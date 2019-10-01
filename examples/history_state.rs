#![recursion_limit = "256"]
#![feature(slice_patterns)]

use std::{convert::Into, fmt, str::FromStr};

use log::info;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use stdweb::{__js_serializable_boilerplate, js_deserializable, js_serializable};
use web_logger;
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew_router::{Route, RouterAgent};

#[derive(SmartDefault, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RouterTarget {
    #[default]
    Home,
    Feed,
    Profile {
        user_id: i64,
    },
    Foo {
        name: String,
        id: i64,
    },
    Post(i64),
    Bar(String, i64),
    Settings(SettingsRoute),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SettingsRoute {
    Notifications,
    Privacy,
    Foobar(i64),
}

impl fmt::Display for RouterTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RouterTarget::Home => "home".into(),
                RouterTarget::Feed => "feed".into(),
                RouterTarget::Profile { user_id } => format!("profile/{}", user_id),
                RouterTarget::Foo { name, id } => format!("foo/{}/{}", name, id),
                RouterTarget::Post(i) => format!("post/{}", i),
                RouterTarget::Bar(s, i) => format!("bar/{}/{}", s, i),
                RouterTarget::Settings(sub_route) => format!("settings/{}", sub_route),
            },
        )
    }
}

impl fmt::Display for SettingsRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SettingsRoute::Notifications => "notifications".into(),
                SettingsRoute::Privacy => "privacy".into(),
                SettingsRoute::Foobar(i) => format!("foobar/{}", i),
            },
        )
    }
}

impl FromStr for RouterTarget {
    type Err = ();

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(match &path_segments(path)[..] {
            ["home"] => RouterTarget::Home,
            ["feed"] => RouterTarget::Feed,
            ["profile", user_id] => RouterTarget::Profile {
                user_id: parse(user_id)?,
            },
            ["foo", name, id] => RouterTarget::Foo {
                name: name.to_string(),
                id: parse(id)?,
            },
            ["post", id] => RouterTarget::Post(parse(id)?),
            ["bar", name, id] => RouterTarget::Bar(name.to_string(), parse(id)?),
            ["settings", sub_route] => RouterTarget::Settings(parse(&sub_route.join("/"))?),
            _ => Err(())?,
        })
    }
}

impl FromStr for SettingsRoute {
    type Err = ();

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(match &path_segments(path)[..] {
            ["notifications"] => SettingsRoute::Notifications,
            ["privacy"] => SettingsRoute::Privacy,
            ["foobar", id] => SettingsRoute::Foobar(parse(id)?),
            _ => Err(())?,
        })
    }
}

fn path_segments(path: &str) -> Vec<&str> {
    path.split('/').filter(|s| !s.is_empty()).collect()
}

fn parse<T: FromStr>(s: &str) -> Result<T, ()> {
    s.parse().map_err(|_| ())
}

js_serializable!(RouterTarget);
js_deserializable!(RouterTarget);

type RouteState = RouterTarget;

const HASH_BASED_URL: bool = true;

/// Convert a RouterTarget into a Route
impl Into<Route<RouteState>> for RouterTarget {
    fn into(self) -> Route<RouteState> {
        let route = self.to_string();
        if HASH_BASED_URL {
            Route {
                fragment: Some(format!("/{}", route)),
                state: self,
                ..Default::default()
            }
        } else {
            Route {
                path_segments: vec![route],
                state: self,
                ..Default::default()
            }
        }
    }
}

/// Convert a Route into a RouterTarget
impl Into<RouterTarget> for Route<RouteState> {
    fn into(self) -> RouterTarget {
        let path = if HASH_BASED_URL {
            self.fragment.unwrap_or_default()
        } else {
            self.path_segments.join("/")
        };
        path.parse().unwrap_or_default()
    }
}

impl Into<PageActions> for RouterTarget {
    fn into(self) -> PageActions {
        PageActions::RoutePage(self.into())
    }
}

// Define a root component that stays consistently mounted even with different routes
pub struct RootComponent {
    // The RouterTarget truct gets created from the routes! macro
    // -> The currently rendered "child_component" is defined as on of RouterTarget::Error or RouterTarget::Login
    child_component: RouterTarget,

    // The RouterAgent is a brige to the Router service created by the yew_router module. See Yew's Agent Model for more details
    router_agent: Box<dyn Bridge<RouterAgent<RouteState>>>,
}

// Define some basic actions that can be executed by elements on our page
pub enum PageActions {
    // The route option contains a route struct
    Route(Route<RouteState>),

    // A RoutePage option for user-generated route changes
    RoutePage(Route<RouteState>),
}

impl Component for RootComponent {
    type Message = PageActions;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        // Connect to the router agent using Yew's bridge  method for workers
        // Send back the method we will be using to route the user
        let mut router_agent = RouterAgent::bridge(link.send_back(PageActions::Route));
        router_agent.send(yew_router::Request::GetCurrentRoute);

        RootComponent {
            child_component: RouterTarget::Feed,
            router_agent,
        }
    }

    fn update(&mut self, interaction: Self::Message) -> ShouldRender {
        // Match against the user interactions
        // This can be any interaction defined in the PageActions enum
        match interaction {
            // Handle all page routing -- this is separated because some routing should not be exposed to the user
            PageActions::RoutePage(pageroute) => self
                .router_agent
                .send(yew_router::Request::ChangeRoute(pageroute)),

            // The Routing event bound to the RouterAgent
            PageActions::Route(route) => {
                info!("{:?}", route);
                self.child_component = route.into();
            }
        }
        true
    }
}

impl Renderable<RootComponent> for RootComponent {
    fn view(&self) -> Html<Self> {
        html! {
            // Let's place a set of html that sticks with the page even when we update the child componenet
            <div>
                <div>
                    <button onclick=|_| RouterTarget::Feed.into()>{ "Click to go to Feed Page" }</button>
                    <div>
                        {
                            for (0..4).map(|i| html! {
                                <button onclick=|_| RouterTarget::Post(i).into()>{
                                    format!("Click to go to page Post({})", i)
                                }</button>
                            })
                        }
                    </div>
                    <div>
                        {
                            for (0..4).map(|i| html! {
                                <button onclick=|_| RouterTarget::Settings(SettingsRoute::Foobar(i)).into()>{
                                    format!("Click to go to page Settings(Foobar({}))", i)
                                }</button>
                            })
                        }
                    </div>
                </div>
                // Render out the child componenet
                { self.child_component.view() }
            </div>
        }
    }
}

impl Renderable<RootComponent> for RouterTarget {
    fn view(&self) -> Html<RootComponent> {
        html! {
            <div> {format!("Current page: {}", self)} </div>
        }
    }
}

fn main() {
    web_logger::init();
    info!("starting up");
    yew::start_app::<RootComponent>();
}
