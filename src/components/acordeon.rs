use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AcordeonCard)]
pub fn acordeon_card() -> Html {
    html! {
    <div class="">
         <h2>
           <div class="flex w-100 items-center py-5 font-medium text-left text-gray-500 border-b border-gray-200 dark:border-gray-700 dark:text-gray-400" >
                                  <span class="">{"1. "}</span>
             <span>{"Numele elevului"}</span>
           </div>
         </h2>
         <div id="accordion-flush-body-1" class="hidden" ariaLabelledby="accordion-flush-heading-1">
           <div class="py-5 font-light border-b border-gray-200 dark:border-gray-700">
             <p class="mb-2 text-gray-500 dark:text-gray-400">{"Flowbite is an open-source library of interactive components built on top of Tailwind CSS including buttons, dropdowns, modals, navbars, and more."}</p>
             <p class="text-gray-500 dark:text-gray-400">{"Check out this guide to learn how to "}</p>
           </div>
         </div>
       </div>
               }
}
