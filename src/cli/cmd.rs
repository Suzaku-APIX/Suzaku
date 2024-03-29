/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::Parser;

use crate::cli::args;
use crate::listener::http::listener::listener as http_license;
use crate::listener::tcp::listener::listener as tcp_listener;

pub fn cli() {
    let app = args::CLi::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(app.log_level.clone()));
    match app.protocol.as_str() {
        "TCP" => tcp_listener(app),
        _ => http_license(app),
    }
}
