// static5/src/components/burger.rs
pub mod burger {

  use yew::prelude::*;

  #[function_component]
  pub fn Burger() -> Html {

    let the_text = "Graph Diagram";
    html! {
      <>
      <div> { the_text } </div>
      <div class="site-container">      
        <svg xmlns="http://www.w3.org/2000/svg" class="hamburger-icon" width="64" height="64" viewBox="0 0 48 48">
          <rect class="top" x="0" y="0" rx="4" ry="4" width="100%" height="8" />
          <rect class="middle" x="0" y="20" rx="4" ry="4" width="100%" height="8" />
          <rect class="bottom" x="0" y="40" rx="4" ry="4" width="100%" height="8" />
        </svg>
      </div>
      </>
    }
  }
}