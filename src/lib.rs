/*
 * lib.rs -- Web API Specification for the AR-MINTIN Application.
 * Copyright (C) 2022 Arnoldas Rauba
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub token: String,
        pub msg: Option<String>,
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NormalResponse {
        pub token: String,
        pub msg: DisplayMessage,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DisplayMessage {
        /// Reference to a topic name
        New {
            topic: String,
        },
        NotifyAssessment,
        Assess(String),
        Display(String, String),
    }
}

pub mod header {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Topic {
        pub title: Option<String>,
        pub body: Option<String>,
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
