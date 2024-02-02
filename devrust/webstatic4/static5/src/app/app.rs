// static5/src/app/app.rs
pub mod app {

  use yew::prelude::*;  
  use crate::components::burger::burger::Burger;
  
  #[function_component]
  pub fn App() -> Html {
      let counter = use_state(|| 0);
      let onclick = {
          let counter = counter.clone();
          move |_| {
              let value = *counter +1;
              counter.set(value);
          }
      };
      html! {
          <>
            <button {onclick} >{ "+1" }</button>
            <p>{ *counter }</p>
            <Burger/>
          </>
      }
  }
  
}