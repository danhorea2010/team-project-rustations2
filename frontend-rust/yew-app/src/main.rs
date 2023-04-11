use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;


#[derive(Clone, PartialEq, Deserialize,Debug)]
pub struct Todo {
    pub id: i64,
    pub description: Option<String>,
    pub title: String,
}


#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let todos = use_state(|| vec![]);
    {
   let todos = todos.clone();
        use_effect_with_deps(move |_| {
            let videos = todos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_todos: Vec<Todo> = Request::get("http://127.0.0.1:8000/api/v1/todo")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_todos);
            });
            || ()
        }, ());
   }
    
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
      <p>{"TODOS:" }</p>
    <div>

    <div style="border: 1px solid #ccc; border-radius: 5px; padding: 20px; margin: 20px; background-color: #fff;">

<table>
    <thead>
      <tr style="padding: 20px">
        <th style="padding:20px;">{"Title"}</th>
        <th style="padding:20px;">{"Description"}</th>
      </tr>
    </thead>
    <tbody>
        {
        todos.iter().map(|todo| {
                html!{<tr key={todo.id}>
                        <td style="padding:20px;">{ format!("{}", &todo.title ) }</td>
                        <td style="padding:20px;">{ format!("{}",  match &todo.description {
                                                                        Some(desc) => desc,
                                                                        None => "",
                                                                } )}</td>
                        </tr>}
            }).collect::<Html>()
    }
    </tbody>
  </table>

   </div>
    </div>

        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
