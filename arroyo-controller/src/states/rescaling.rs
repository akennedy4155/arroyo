use arroyo_rpc::grpc;

use crate::JobMessage;

use super::{scheduling::Scheduling, stopping::Stopping, Context, State, StateError, Transition};

#[derive(Debug)]
pub struct Rescaling {}

#[async_trait::async_trait]
impl State for Rescaling {
    fn name(&self) -> &'static str {
        "Rescaling"
    }

    async fn next(mut self: Box<Self>, ctx: &mut Context) -> Result<Transition, StateError> {
        let job_controller = ctx.job_controller.as_mut().unwrap();

        if let Err(e) = job_controller.checkpoint(true).await {
            return Err(ctx.retryable(self, "failed to initiate final checkpoint", e, 10));
        }

        loop {
            match job_controller.checkpoint_finished().await {
                Ok(done) => {
                    if done && job_controller.finished() {
                        return Ok(Transition::next(*self, Scheduling {}));
                    }
                }
                Err(e) => {
                    return Err(ctx.retryable(
                        self,
                        "failed while monitoring final checkpoint",
                        e,
                        10,
                    ));
                }
            }

            match ctx.rx.recv().await.expect("channel closed while receiving") {
                JobMessage::RunningMessage(msg) => {
                    if let Err(e) = job_controller.handle_message(msg).await {
                        return Err(ctx.retryable(
                            self,
                            "failed while waiting for job finish",
                            e,
                            10,
                        ));
                    }
                }
                JobMessage::ConfigUpdate(c) => {
                    match c.stop_mode {
                        crate::types::public::StopMode::immediate => {
                            return Ok(Transition::next(
                                *self,
                                Stopping {
                                    stop_mode: grpc::StopMode::Immediate,
                                },
                            ));
                        }
                        crate::types::public::StopMode::force => {
                            todo!("implement force stop mode");
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                _ => {
                    // ignore other messages
                }
            }
        }
    }
}
