#[cfg(test)]
mod tests {
    use arroyo_sql_macro::single_test_codegen;
    use chrono;

    // Casts
    single_test_codegen!(
        "cast_i64_f32",
        "CAST(nullable_i64 as FLOAT)",
        arroyo_sql::TestStruct {
            nullable_i64: Some(5),
            ..Default::default()
        },
        Some(5f32)
    );

    single_test_codegen!(
        "cast_null",
        "CAST(nullable_i64 as FLOAT)",
        arroyo_sql::TestStruct {
            nullable_i64: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "cast_string_to_f32",
        "CAST(nullable_string as FLOAT)",
        arroyo_sql::TestStruct {
            nullable_string: Some("1.25".to_string()),
            ..Default::default()
        },
        Some(1.25f32)
    );

    single_test_codegen!(
        "cast_f64_to_string",
        "CAST(nullable_f64 as STRING)",
        arroyo_sql::TestStruct {
            nullable_f64: Some(1.25),
            ..Default::default()
        },
        Some("1.25".to_string())
    );

    single_test_codegen!(
        "cast_timestamp_to_string",
        "CAST(non_nullable_timestamp as STRING)",
        arroyo_sql::TestStruct::default(),
        "1970-01-01T00:00:00+00:00".to_string()
    );
    single_test_codegen!(
        "cast_null_timestamp_to_string",
        "CAST(nullable_timestamp as STRING)",
        arroyo_sql::TestStruct::default(),
        None
    );
    // Category: Math - Addition

    // Test case: Non-nullable and nullable values, nullable is non-null
    single_test_codegen!(
        "one_plus_nullable_two",
        "non_nullable_i32 + nullable_i64",
        arroyo_sql::TestStruct {
            non_nullable_i32: 1,
            nullable_i64: Some(2),
            ..Default::default()
        },
        Some(3i64)
    );

    // Test case: Non-nullable and nullable values, nullable is null
    single_test_codegen!(
        "one_plus_null",
        "non_nullable_i32 + nullable_i64",
        arroyo_sql::TestStruct {
            non_nullable_i32: 1,
            nullable_i64: None,
            ..Default::default()
        },
        None
    );

    // Test case: Nullable and non-nullable values, nullable is non-null
    single_test_codegen!(
        "nullable_two_plus_one",
        "nullable_i32 + non_nullable_i64",
        arroyo_sql::TestStruct {
            nullable_i32: Some(1),
            non_nullable_i64: 2,
            ..Default::default()
        },
        Some(3i64)
    );

    // Test case: Nullable and non-nullable values, nullable is null
    single_test_codegen!(
        "null_plus_two",
        "nullable_i32 + non_nullable_i64",
        arroyo_sql::TestStruct {
            nullable_i32: None,
            non_nullable_i64: 2,
            ..Default::default()
        },
        None
    );

    // Category: Math - Subtraction

    // Test case: Non-nullable and nullable values, nullable is non-null
    single_test_codegen!(
        "one_minus_nullable_two",
        "non_nullable_i32 - nullable_i64",
        arroyo_sql::TestStruct {
            non_nullable_i32: 1,
            nullable_i64: Some(2),
            ..Default::default()
        },
        Some(-1i64)
    );

    // Test case: Non-nullable and nullable values, nullable is null
    single_test_codegen!(
        "one_minus_null",
        "non_nullable_i32 - nullable_i64",
        arroyo_sql::TestStruct {
            non_nullable_i32: 1,
            nullable_i64: None,
            ..Default::default()
        },
        None
    );

    // Test case: Nullable and non-nullable values, nullable is non-null
    single_test_codegen!(
        "nullable_two_minus_one",
        "nullable_i32 - non_nullable_i64",
        arroyo_sql::TestStruct {
            nullable_i32: Some(2),
            non_nullable_i64: 1,
            ..Default::default()
        },
        Some(1i64)
    );

    // Test case: Nullable and non-nullable values, nullable is null
    single_test_codegen!(
        "null_minus_two",
        "nullable_i32 - non_nullable_i64",
        arroyo_sql::TestStruct {
            nullable_i32: None,
            non_nullable_i64: 2,
            ..Default::default()
        },
        None
    );
    // Category: Math - Modulo

    // Test case: Non-nullable and nullable values, nullable is non-null
    single_test_codegen!(
        "one_modulo_nullable_two",
        "non_nullable_i32 % nullable_i64",
        arroyo_sql::TestStruct {
            non_nullable_i32: 1,
            nullable_i64: Some(2),
            ..Default::default()
        },
        Some(1i64)
    );

    // Test case: Non-nullable and nullable values, nullable is null
    single_test_codegen!(
        "one_modulo_null",
        "non_nullable_i32 % nullable_i64",
        arroyo_sql::TestStruct {
            non_nullable_i32: 1,
            nullable_i64: None,
            ..Default::default()
        },
        None
    );

    // Test case: Nullable and non-nullable values, nullable is non-null
    single_test_codegen!(
        "nullable_two_modulo_one",
        "nullable_i32 % non_nullable_i64",
        arroyo_sql::TestStruct {
            nullable_i32: Some(2),
            non_nullable_i64: 1,
            ..Default::default()
        },
        Some(0i64)
    );

    // Test case: Nullable and non-nullable values, nullable is null
    single_test_codegen!(
        "null_modulo_two",
        "nullable_i32 % non_nullable_i64",
        arroyo_sql::TestStruct {
            nullable_i32: None,
            non_nullable_i64: 2,
            ..Default::default()
        },
        None
    );
    // Category: Unary Operators

    // IS NOT NULL
    single_test_codegen!(
        "non_nullable_bool_is_not_null",
        "non_nullable_bool IS NOT NULL",
        arroyo_sql::TestStruct {
            non_nullable_bool: true,
            ..Default::default()
        },
        true
    );

    single_test_codegen!(
        "nullable_bool_is_not_null",
        "nullable_bool IS NOT NULL",
        arroyo_sql::TestStruct {
            nullable_bool: Some(true),
            ..Default::default()
        },
        true
    );

    // IS NULL
    single_test_codegen!(
        "nullable_bool_is_null",
        "nullable_bool IS NULL",
        arroyo_sql::TestStruct {
            nullable_bool: None,
            ..Default::default()
        },
        true
    );

    // IS TRUE
    single_test_codegen!(
        "non_nullable_bool_is_true",
        "non_nullable_bool IS TRUE",
        arroyo_sql::TestStruct {
            non_nullable_bool: true,
            ..Default::default()
        },
        true
    );

    single_test_codegen!(
        "nullable_bool_is_true",
        "nullable_bool IS TRUE",
        arroyo_sql::TestStruct {
            nullable_bool: Some(true),
            ..Default::default()
        },
        true
    );

    // IS FALSE
    single_test_codegen!(
        "non_nullable_bool_is_false",
        "non_nullable_bool IS FALSE",
        arroyo_sql::TestStruct {
            non_nullable_bool: false,
            ..Default::default()
        },
        true
    );

    single_test_codegen!(
        "nullable_bool_is_false",
        "nullable_bool IS FALSE",
        arroyo_sql::TestStruct {
            nullable_bool: Some(false),
            ..Default::default()
        },
        true
    );

    // IS UNKNOWN
    single_test_codegen!(
        "nullable_bool_is_unknown",
        "nullable_bool IS UNKNOWN",
        arroyo_sql::TestStruct {
            nullable_bool: None,
            ..Default::default()
        },
        true
    );

    // IS NOT TRUE
    single_test_codegen!(
        "non_nullable_bool_is_not_true",
        "non_nullable_bool IS NOT TRUE",
        arroyo_sql::TestStruct {
            non_nullable_bool: false,
            ..Default::default()
        },
        true
    );

    single_test_codegen!(
        "nullable_bool_is_not_true",
        "nullable_bool IS NOT TRUE",
        arroyo_sql::TestStruct {
            nullable_bool: Some(false),
            ..Default::default()
        },
        true
    );

    // IS NOT FALSE
    single_test_codegen!(
        "non_nullable_bool_is_not_false",
        "non_nullable_bool IS NOT FALSE",
        arroyo_sql::TestStruct {
            non_nullable_bool: true,
            ..Default::default()
        },
        true
    );

    // Is not false
    single_test_codegen!(
        "nullable_bool_is_not_false",
        "nullable_bool IS NOT FALSE",
        arroyo_sql::TestStruct {
            nullable_bool: Some(true),
            ..Default::default()
        },
        true
    );

    // Math functions

    // Sqrt
    single_test_codegen!(
        "sqrt_of_four",
        "sqrt(non_nullable_i64) < 3.01",
        arroyo_sql::TestStruct {
            non_nullable_i64: 8,
            non_nullable_f64: 2.0,
            ..Default::default()
        },
        true
    );

    // String functions

    single_test_codegen!(
        "ascii",
        "ascii(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "test".into(),
            ..Default::default()
        },
        't' as i32
    );
    single_test_codegen!(
        "ascii_empty_string",
        "ascii(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "".into(),
            ..Default::default()
        },
        0
    );

    single_test_codegen!(
        "ascii_on_null",
        "ascii(nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );
    // BitLength
    single_test_codegen!(
        "bit_length_null",
        "bit_length(nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "bit_length_empty_string",
        "bit_length(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "".into(),
            ..Default::default()
        },
        0
    );

    // Btrim
    single_test_codegen!(
        "btrim_null",
        "btrim(nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "btrim_empty_string",
        "btrim(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "".into(),
            ..Default::default()
        },
        "".to_string()
    );

    // CharacterLength
    single_test_codegen!(
        "character_length_null",
        "character_length(nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "character_length_empty_string",
        "character_length(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "".into(),
            ..Default::default()
        },
        0
    );

    // Chr
    single_test_codegen!(
        "chr_null",
        "chr(nullable_i32)",
        arroyo_sql::TestStruct {
            nullable_i32: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "chr_corner_case",
        "chr(non_nullable_i32)",
        arroyo_sql::TestStruct {
            non_nullable_i32: 65,
            ..Default::default()
        },
        "A".to_string()
    );
    // Concat
    single_test_codegen!(
        "concat_strings",
        "concat(non_nullable_string, ', ', nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello".into(),
            nullable_string: Some("World".into()),
            ..Default::default()
        },
        "Hello, World"
    );

    single_test_codegen!(
        "concat_string_and_null",
        "concat(non_nullable_string, nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello".into(),
            nullable_string: None,
            ..Default::default()
        },
        "Hello"
    );

    single_test_codegen!(
        "concat_null_and_string",
        "concat(nullable_string, non_nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            non_nullable_string: "World".into(),
            ..Default::default()
        },
        "World"
    );

    single_test_codegen!(
        "concat_empty_strings",
        "concat(non_nullable_string, nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "".into(),
            nullable_string: Some("".into()),
            ..Default::default()
        },
        ""
    );

    single_test_codegen!(
        "concat_null_and_null",
        "concat(nullable_string, nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        ""
    );

    // ConcatWithSeparator
    single_test_codegen!(
        "concat_with_separator",
        "concat_ws(', ', non_nullable_string, nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello".into(),
            nullable_string: Some("World".into()),
            ..Default::default()
        },
        "Hello, World".to_string()
    );

    single_test_codegen!(
        "concat_with_separator_null",
        "concat_ws(', ', non_nullable_string, nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello".into(),
            nullable_string: None,
            ..Default::default()
        },
        "Hello".to_string()
    );

    // InitCap
    single_test_codegen!(
        "init_cap",
        "initcap(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "hello world".into(),
            ..Default::default()
        },
        "Hello World".to_string()
    );

    single_test_codegen!(
        "init_cap_null",
        "initcap(nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    // SplitPart
    single_test_codegen!(
        "split_part",
        "split_part(non_nullable_string, ', ', 2)",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello, World, Test".into(),
            ..Default::default()
        },
        "World".to_string()
    );

    single_test_codegen!(
        "split_part_null",
        "split_part(nullable_string, ', ', 2)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    // StartsWith
    single_test_codegen!(
        "starts_with",
        "starts_with(non_nullable_string, 'Hel')",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello, World".into(),
            ..Default::default()
        },
        true
    );

    single_test_codegen!(
        "starts_with_false",
        "starts_with(non_nullable_string, 'Wor')",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello, World".into(),
            ..Default::default()
        },
        false
    );

    single_test_codegen!(
        "starts_with_null",
        "starts_with(nullable_string, 'Hel')",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    // Hash
    single_test_codegen!(
        "md5_non_null",
        "md5(non_nullable_string)",
        arroyo_sql::TestStruct {
            non_nullable_string: "Hello, World".into(),
            ..Default::default()
        },
        "82bb413746aee42f89dea2b59614f9ef".to_string()
    );

    single_test_codegen!(
        "md5_null",
        "md5(nullable_string)",
        arroyo_sql::TestStruct {
            nullable_string: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "sha224_non_null",
        "sha224(non_nullable_bytes)",
        arroyo_sql::TestStruct {
            non_nullable_bytes: "asdf".as_bytes().to_vec(),
            ..Default::default()
        },
        hex::decode("7872a74bcbf298a1e77d507cd95d4f8d96131cbbd4cdfc571e776c8a").unwrap()
    );

    single_test_codegen!(
        "sha224_null",
        "sha224(nullable_bytes)",
        arroyo_sql::TestStruct {
            nullable_bytes: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "sha256_non_null",
        "sha256(non_nullable_bytes)",
        arroyo_sql::TestStruct {
            non_nullable_bytes: "asdf".as_bytes().to_vec(),
            ..Default::default()
        },
        hex::decode("f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b").unwrap()
    );

    single_test_codegen!(
        "sha256_null",
        "sha256(nullable_bytes)",
        arroyo_sql::TestStruct {
            nullable_bytes: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "sha384_non_null",
        "sha384(non_nullable_bytes)",
        arroyo_sql::TestStruct {
            non_nullable_bytes: "asdf".as_bytes().to_vec(),
            ..Default::default()
        },
        hex::decode("a69e7df30b24c042ec540ccbbdbfb1562c85787038c885749c1e408e2d62fa36642cd0075fa351e822e2b8a59139cd9d").unwrap()
    );

    single_test_codegen!(
        "sha384_null",
        "sha384(nullable_bytes)",
        arroyo_sql::TestStruct {
            nullable_bytes: None,
            ..Default::default()
        },
        None
    );

    single_test_codegen!(
        "sha512_non_null",
        "sha512(non_nullable_bytes)",
        arroyo_sql::TestStruct {
            non_nullable_bytes: "asdf".as_bytes().to_vec(),
            ..Default::default()
        },
        hex::decode("401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b3727429080fb337591abd3e44453b954555b7a0812e1081c39b740293f765eae731f5a65ed1").unwrap()
    );

    single_test_codegen!(
        "sha512_null",
        "sha512(nullable_bytes)",
        arroyo_sql::TestStruct {
            nullable_bytes: None,
            ..Default::default()
        },
        None
    );
}
