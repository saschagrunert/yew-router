#![recursion_limit="256"]

#[macro_use]
extern crate yew_router;
use yew_router::{Request, Route, RouterAgent};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender, Bridge, Bridged};


// Define application routes via the macro
// This creates the `RouterTarget` struct
// Error is a required route according to the router macro
routes! {
    Error =>  "/error",
    Login =>  "/login",
}

// Define a root component that stays consistently mounted even with different routes
pub struct RootComponent {
    // The RouterTarget truct gets created from the routes! macro
    // -> The currently rendered "child_component" is defined as on of RouterTarget::Error or RouterTarget::Login
    child_component: RouterTarget,

    // The RouterAgent is a brige to the Router service created by the yew_router module. See Yew's Agent Model for more details
    router_agent: Box<Bridge<RouterAgent<()>>>,
}

// Define some basic actions that can be executed by elements on our page
pub enum PageActions {
    // The route option contains a route struct
    Route(Route<()>), 

    // A RoutePage option for user-generated route changes
    RoutePage(Route<()>), 
}



impl Component for RootComponent {
    type Message = PageActions;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        // Connect to the router agent using Yew's bridge  method for workers
        // Send back the method we will be using to route the user
        let router_agent = RouterAgent::bridge(link.send_back(PageActions::Route));

        RootComponent {
            child_component: RouterTarget::Login,
            router_agent: router_agent
        }
    }

    fn update(&mut self, interaction: Self::Message) -> ShouldRender {
        // Match against the user interactions
        // This can be any interaction defined in the PageActions enum
        match interaction {
            // Handle all page routing -- this is separated because some routing should not be exposed to the user 
            PageActions::RoutePage(pageroute) => self.router_agent.send(yew_router::Request::ChangeRoute(pageroute)),

            // The Routing event bound to the RouterAgent
            PageActions::Route(route) => self.child_component = route.into(),
        
            _ => {},
        }
        true
    }
}

impl Renderable<RootComponent> for RootComponent {
    fn view(&self) -> Html<Self> {
        html!{
            // Let's place a set of html that sticks with the page even when we update the child componenet
            <div>
                <div>
                    <button onclick=|_| PageActions::RoutePage(RouterTarget::Login.into())>{ "Click to go to Login Page" }</button>
                    <button onclick=|_| PageActions::RoutePage(RouterTarget::Error.into())>{ "Click to go to Error Page" }</button>
                </div>
                // Render out the child componenet                    
                { self.child_component.view() }
            </div>        
        }
    }
}

impl Renderable<RootComponent> for RouterTarget {
    fn view(&self) -> Html<RootComponent> {
        match *self {
            // Html to render based on the child componenet enum view
            RouterTarget::Login => {
                html! {        
                        <div> {"Login page activated"} </div>
                }
            },
            RouterTarget::Error => {
                html! {
                    <div> {"Error page activated"} </div>
                }
            },
            _ => {html! {} },
        }
    }
}

fn main() {
    yew::start_app::<RootComponent>();
}