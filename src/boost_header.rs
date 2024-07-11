use rocket::http::Header;

/// Enum representing Boost headers
#[derive(Debug, Clone)]
pub enum BoostHeader {
    /// Custom header for arbitrary values (Header name, Header value)
    Custom(String, String),
    /// Allows you to do a client-side redirect that does not do a full page reload
    Location(String),
    /// Pushes a new url into the history stack
    PushUrl(String),
    /// Can be used to do a client-side redirect to a new location
    Redirect(String),
    /// If set the client-side will do a full refresh of the page
    Refresh,
    /// Replaces the current URL in the location bar
    ReplaceUrl(String),
    /// Allows you to specify how the response will be swapped. See hx-swap for possible values
    Reswap(String),
    /// A CSS selector that updates the target of the content update to a different element on the page
    Retarget(String),
    /// A CSS selector that allows you to choose which part of the response is used to be swapped in. Overrides an existing hx-select on the triggering element
    Reselect(String),
    /// Allows you to trigger client-side events
    Trigger(String),
    /// Allows you to trigger client-side events after the settle step
    TriggerAfterSettle(String),
    /// Allows you to trigger client-side events after the swap step
    TriggerAfterSwap(String),
}

impl From<&BoostHeader> for Header<'static> {
    fn from(boost_header: &BoostHeader) -> Header<'static> {
        match boost_header.clone() {
            BoostHeader::Custom(name, value) => Header::new(name, value),
            BoostHeader::Location(value) => Header::new("HX-Location", value),
            BoostHeader::PushUrl(value) => Header::new("HX-Push-Url", value),
            BoostHeader::Redirect(value) => Header::new("HX-Redirect", value),
            BoostHeader::Refresh => Header::new("HX-Refresh", "true"),
            BoostHeader::ReplaceUrl(value) => Header::new("HX-Replace-Url", value),
            BoostHeader::Reswap(value) => Header::new("HX-Reswap", value),
            BoostHeader::Retarget(value) => Header::new("HX-Retarget", value),
            BoostHeader::Reselect(value) => Header::new("HX-Reselect", value),
            BoostHeader::Trigger(value) => Header::new("HX-Trigger", value),
            BoostHeader::TriggerAfterSettle(value) => Header::new("HX-Trigger-After-Settle", value),
            BoostHeader::TriggerAfterSwap(value) => Header::new("HX-Trigger-After-Swap", value),
        }
    }
}
