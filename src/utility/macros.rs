/// A macro for implementing the `Daemon` trait for MCP servers.
///
/// By applying this attribute to `impl Daemon for ...` and specifying the attributes listed in the [Methods section](#methods) to methods,
/// you can implement a Model Context Protocol server.
///
// #[include_doc("../../../README.md",start("### Example"))]
/// ### Example
///
/// ```rust,ignore
/// use std::sync::Mutex;
///
/// use mcp_daemon::server::{server, Daemon, serve_stdio};
/// use mcp_daemon::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     serve_stdio(ExampleDaemon(Mutex::new(ServerData { count: 0 }))).await?;
///     Ok(())
/// }
///
/// struct ExampleDaemon(Mutex<ServerData>);
///
/// struct ServerData {
///   /// Server state
///   count: u32,
/// }
///
/// #[mcp_server]
/// impl Daemon for ExampleDaemon {
///     /// Description sent to MCP client
///     #[prompt]
///     async fn example_prompt(&self) -> Result<&str> {
///         Ok("Hello!")
///     }
///
///     #[resource("my_app://files/{name}.txt")]
///     async fn read_file(&self, name: String) -> Result<String> {
///         Ok(format!("Content of {name}.txt"))
///     }
///
///     #[tool]
///     async fn add_count(&self, message: String) -> Result<String> {
///         let mut state = self.0.lock().unwrap();
///         state.count += 1;
///         Ok(format!("Echo: {message} {}", state.count))
///     }
/// }
/// ```
// #[include_doc("../../../README.md",end("## Support Status"))]
// #[include_doc("../../../README.md",start("### Methods"))]
/// ### Methods
///
/// | Attribute                  | [`Daemon`] methods                                                    | Model context protocol methods                                           |
/// | -------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------ |
/// | [`#[prompt]`](#prompt)     | [`prompts_list`]<br>[`prompts_get`]                                      | [`prompts/list`]<br>[`prompts/get`]                                      |
/// | [`#[resource]`](#resource) | [`resources_list`]<br>[`resources_read`]<br>[`resources_templates_list`] | [`resources/list`]<br>[`resources/read`]<br>[`resources/templates/list`] |
/// | [`#[tool]`](#tool)         | [`tools_list`]<br>[`tools_call`]                                         | [`tools/list`]<br>[`tools/call`]                                         |
// #[include_doc("../../../README.md",end("## Usage"))]
// #[include_doc("../../../README.md",start("## Attribute Descriptions"))]
/// ## Attribute Descriptions
///
/// ### `#[prompt]`
///
/// ```rust,ignore
/// #[prompt("name")]
/// async fn func_name(&self) -> Result<GetPromptResult> { }
/// ```
///
/// - "name" (optional): Prompt name. If omitted, the function name is used.
///
/// Implements the following methods:
///
/// - [`prompts_list`]
/// - [`prompts_get`]
///
/// Function arguments become prompt arguments. Arguments must implement the following trait:
///
/// - [`FromStr`]: Trait for restoring values from strings
///
/// Arguments can be given names using the `#[arg("name")]` attribute.
/// If not specified, the name used is the function argument name with leading `_` removed.
///
/// Return value: [`Result<impl Into<GetPromptResult>>`]
///
/// ```rust,ignore
/// use mcp_daemon::Result;
/// use mcp_daemon::server::{server, Daemon};
///
/// struct ExampleDaemon;
///
/// #[mcp_server]
/// impl Daemon for ExampleDaemon {
///   /// Function description (for AI)
///   #[prompt]
///   async fn hello(&self) -> Result<&str> {
///     Ok("Hello, world!")
///   }
///
///   #[prompt]
///   async fn echo(&self,
///     /// Argument description (for AI)
///     a: String,
///     /// Argument description (for AI)
///     #[arg("x")]
///     b: String,
///   ) -> Result<String> {
///     Ok(format!("Hello, {a} {b}!"))
///   }
/// }
/// ```
///
/// ### `#[resource]`
///
/// ```rust,ignore
/// #[resource("url_template", name = "name", mime_type = "mime_type")]
/// async fn func_name(&self) -> Result<ReadResourceResult> { }
/// ```
///
/// - "url_template" (optional): URI Template ([RFC 6570]) indicating the URL of resources this method handles. If omitted, handles all URLs.
/// - "name" (optional): Resource name. If omitted, the function name is used.
/// - "mime_type" (optional): MIME type of the resource.
///
/// Implements the following methods:
///
/// - [`resources_list`] (can be manually implemented)
/// - [`resources_read`]
/// - [`resources_templates_list`]
///
/// Function arguments become URI Template variables. Arguments must implement the following trait:
///
/// - [`FromStr`]: Trait for restoring values from strings
///
/// URI Templates are specified in [RFC 6570] Level2. The following variables can be used in URI Templates:
///
/// - `{var}`
/// - `{+var}`
/// - `{#var}`
///
/// Return value: [`Result<impl Into<ReadResourceResult>>`]
///
/// ```rust,ignore
/// use mcp_daemon::Result;
/// use mcp_daemon::server::{server, Daemon};
///
/// struct ExampleDaemon;
///
/// #[mcp_server]
/// impl Daemon for ExampleDaemon {
///   /// Function description (for AI)
///   #[resource("my_app://x/y.txt")]
///   async fn file_one(&self) -> Result<String> {
///     Ok(format!("one file"))
///   }
///
///   #[resource("my_app://{a}/{+b}")]
///   async fn file_ab(&self, a: String, b: String) -> Result<String> {
///     Ok(format!("{a} and {b}"))
///   }
///
///   #[resource]
///   async fn file_any(&self, url: String) -> Result<String> {
///     Ok(format!("any file"))
///   }
/// }
/// ```
///
/// The automatically implemented [`resources_list`] returns a list of URLs without variables specified in the `#[resource]` attribute.
/// If you need to return other URLs, you must manually implement `resources_list`.
/// If `resources_list` is manually implemented, it is not automatically implemented.
///
/// ### `#[tool]`
///
/// ```rust,ignore
/// #[tool("name")]
/// async fn func_name(&self) -> Result<CallToolResult> { }
/// ```
///
/// - "name" (optional): Tool name. If omitted, the function name is used.
///
/// Implements the following methods:
///
/// - [`tools_list`]
/// - [`tools_call`]
///
/// Function arguments become tool arguments. Arguments must implement all of the following traits:
///
/// - [`DeserializeOwned`]: Trait for restoring values from JSON
/// - [`JsonSchema`]: Trait for generating JSON Schema (JSON Schema is sent to MCP Client so AI can understand argument structure)
///
/// Arguments can be given names using the `#[arg("name")]` attribute.
/// If not specified, the name used is the function argument name with leading `_` removed.
///
/// Return value: [`Result<impl Into<CallToolResult>>`]
///
/// ```rust,ignore
/// use mcp_daemon::Result;
/// use mcp_daemon::server::{server, Daemon};
///
/// struct ExampleDaemon;
///
/// #[server]
/// impl Daemon for ExampleDaemon {
///   /// Function description (for AI)
///   #[tool]
///   async fn echo(&self,
///     /// Argument description (for AI)
///     a: String,
///     /// Argument description (for AI)
///     #[arg("x")]
///     b: String,
///   ) -> Result<String> {
///     Ok(format!("Hello, {a} {b}!"))
///   }
/// }
/// ```
///
/// ### Manual Implementation
///
/// You can also directly implement `Daemon` methods without using attributes.
///
/// Additionally, the following methods do not support implementation through attributes and must be implemented manually:
///
/// - [`server_info`]
/// - [`instructions`]
/// - [`completion_complete`]
///
/// The following method can be overridden with manual implementation over the attribute-based implementation:
///
/// - [`resources_list`]
// #[include_doc("../../../README.md",end("## Testing"))]
///
/// [Model Context Protocol]: https://spec.modelcontextprotocol.io/specification/2025-03-26/
/// [RFC 6570]: https://www.rfc-editor.org/rfc/rfc6570.html
/// [`prompts/list`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/prompts/#listing-prompts
/// [`prompts/get`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/prompts/#getting-a-prompt
/// [`resources/list`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/resources/#listing-resources
/// [`resources/read`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/resources/#reading-resources
/// [`resources/templates/list`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/resources/#resource-templates
/// [`tools/list`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/tools/#listing-tools
/// [`tools/call`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/server/tools/#calling-a-tool
/// [`roots/list`]: https://spec.modelcontextprotocol.io/specification/2025-03-26/client/roots/#listing-roots
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`JsonSchema`]: https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html
/// [`DeserializeOwned`]: https://docs.rs/serde/latest/serde/de/trait.DeserializeOwned.html
/// [`Daemon`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html
/// [`Client`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/client/struct.Client.html
/// [`prompts_list`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.prompts_list
/// [`prompts_get`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.prompts_get
/// [`resources_list`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.resources_list
/// [`resources_read`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.resources_read
/// [`resources_templates_list`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.resources_templates_list
/// [`tools_list`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/client/struct.McpClient.html#method.tools_list
/// [`tools_call`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/client/struct.McpClient.html#method.tools_call
/// [`GetPromptResult`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/schema/struct.GetPromptResult.html
/// [`ReadResourceResult`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/schema/struct.ReadResourceResult.html
/// [`CallToolResult`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/schema/struct.CallToolResult.html
/// [`mcp_daemon::Error`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/struct.Error.html
/// [`mcp_daemon::Result`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/type.Result.html
/// [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
/// [`std::error::Error + Sync + Send + 'static`]: https://doc.rust-lang.org/std/error/trait.Error.html
/// [`anyhow::bail!`]: https://docs.rs/anyhow/latest/anyhow/macro.bail.html
/// [`bail!`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/macro.bail.html
/// [`bail_public!`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/macro.bail_public.html
/// [`server_info`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.server_info
/// [`instructions`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.instructions
/// [`completion_complete`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/trait.Daemon.html#method.completion_complete
/// [`Result<impl Into<GetPromptResult>>`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/schema/struct.GetPromptResult.html
/// [`Result<impl Into<ReadResourceResult>>`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/schema/struct.ReadResourceResult.html
/// [`Result<impl Into<CallToolResult>>`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/schema/struct.CallToolResult.html
/// [`RequestContext`]: https://docs.rs/mcp_daemon/latest/mcp_daemon/server/struct.RequestContext.html
pub use crate::server;
