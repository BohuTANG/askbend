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
use chrono::DateTime;
use chrono::Utc;
use databend_driver::new_connection;
use databend_driver::Connection;

use crate::base::escape_sql_string;
use crate::Config;

#[derive(Clone)]
pub struct DatabendDriver {
    pub database: String,
    pub table: String,
    pub answer_table: String,
    pub min_content_length: usize,
    pub max_content_length: usize,
    pub top: usize,
    pub min_distance: f32,
    pub product: String,
    pub prompt_template: String,
    pub conn: Box<dyn Connection>,
}

impl DatabendDriver {
    pub fn connect(conf: &Config) -> Result<Self> {
        let conn = new_connection(&conf.database.dsn)?;
        Ok(DatabendDriver {
            database: conf.database.database.clone(),
            table: conf.database.table.clone(),
            answer_table: conf.database.answer_table.clone(),
            min_content_length: conf.query.min_content_length,
            max_content_length: conf.query.max_content_length,
            top: conf.query.top,
            min_distance: conf.query.min_distance.parse::<f32>().unwrap_or(0.28),
            product: conf.query.product.to_string(),
            prompt_template: conf.query.prompt.to_string(),
            conn,
        })
    }

    pub async fn insert_answer(
        &self,
        query: &str,
        prompt: &str,
        similar_distances: &[f32],
        similar_sections: &str,
        answer: &str,
    ) -> Result<()> {
        if self.answer_table.is_empty() {
            return Ok(());
        }

        let now: DateTime<Utc> = Utc::now();
        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let sql = format!(
            "INSERT INTO {}.{} (question, prompt, similar_distances, similar_sections, answer, ts) VALUES ('{}','{}', {:?}, '{}', '{}', '{}')",
            self.database,
            self.answer_table,
            escape_sql_string(query),
            escape_sql_string(prompt),
            similar_distances,
            escape_sql_string(similar_sections),
            escape_sql_string(answer),
            now_str,
        );
        let _ = self.conn.exec(&sql).await?;
        Ok(())
    }
}
