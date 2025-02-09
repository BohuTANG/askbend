// Copyright 2023 Databend Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use askbend::FileOperator;

#[test]
pub fn test_list() -> Result<()> {
    let operator = FileOperator::create("tests/testdata/", "md", &[]);
    let metas = operator.list()?;
    println!("{:?}", metas);
    assert!(metas.len() == 3);
    assert!(metas[0].file_name == "1.md");
    assert_eq!(metas[0].dir, "tests/testdata");
    assert_eq!(metas[0].full_path, "tests/testdata/1.md");

    Ok(())
}

#[test]
pub fn test_list_ignore_dir() -> Result<()> {
    let operator = FileOperator::create("tests/testdata/", "md", &["ignore_dir".to_string()]);
    let metas = operator.list()?;
    println!("{:?}", metas);
    assert!(metas.len() == 2);

    Ok(())
}
