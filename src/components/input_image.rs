use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(FileInput)]
pub fn file_input() -> Html {
    // let oninput = {
    //     let current_username = username.clone();

    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         current_username.set(input.value());
    //     })
    // };

    html! {
    	<div class="flex justify-center items-center w-2/3">
    	  <label
    	    htmlFor="dropzone-file"
    	    class="flex flex-col justify-center items-center w-full h-64 bg-gray-50 rounded-lg border-2 border-gray-300 border-dashed cursor-pointer dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
    	  	>
    
					<div class="flex flex-col justify-center items-center pt-5 pb-6">
						<svg
							ariaHidden="true"
							class="mb-3 w-10 h-10 text-gray-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								strokeLinecap="round"
								strokeLinejoin="round"
								strokeWidth="10"
								d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
							></path>
						</svg>
						<p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
							<span class="font-semibold">{"Click pentru a încărca imagini"}</span> {" sau trage imaginile în această zonă"}
						</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">
							{"PNG, JPG (MAX. 800x400px)"}
						</p>
					</div>

					<input
						id="dropzone-file"
						type="file"
						accept="image/*"
						class="hidden"
						multiple=true
						// onChange={(e) => upload(e)}
					/>
     	 	</label>
    	</div>
        }
}
