use yew::functional::*;
use yew::prelude::*;

#[function_component(AcordeonCard)]
pub fn acordeon_card() -> Html {
    let collapsed = use_state(|| false);

    let onclick = {
        let curr_collapsed = collapsed.clone();

        Callback::from(move |_e: MouseEvent| match *curr_collapsed {
            true => curr_collapsed.set(false),
            false => curr_collapsed.set(true),
        })
    };
    html! {
          <div id="accordion-collapse" dataAccordion="collapse">
            <h2 id="accordion-collapse-heading-3">
              <button type="button" {onclick} class="flex items-center justify-between w-full p-5 font-medium text-left text-gray-500 border border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800" dataAccordionTarget="#accordion-collapse-body-3" ariaExpanded="false" ariaControls="accordion-collapse-body-3">
                <span>{"1. Cazacu Radu - 4 imagini"}</span>

                <div class="flex">
                 <svg ariaHidden="true" class="hover:text-white -rotate-90 ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
                 <svg ariaHidden="true" class="hover:text-white rotate-90 ml-2 mr-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>



                    <svg class="svg-icon w-6 h-6 mr-2 hover:text-white" viewBox="0 0 20 20" fill="currentColor">
                      <path d="M10.185,1.417c-4.741,0-8.583,3.842-8.583,8.583c0,4.74,3.842,8.582,8.583,8.582S18.768,14.74,18.768,10C18.768,5.259,14.926,1.417,10.185,1.417 M10.185,17.68c-4.235,0-7.679-3.445-7.679-7.68c0-4.235,3.444-7.679,7.679-7.679S17.864,5.765,17.864,10C17.864,14.234,14.42,17.68,10.185,17.68 M10.824,10l2.842-2.844c0.178-0.176,0.178-0.46,0-0.637c-0.177-0.178-0.461-0.178-0.637,0l-2.844,2.841L7.341,6.52c-0.176-0.178-0.46-0.178-0.637,0c-0.178,0.176-0.178,0.461,0,0.637L9.546,10l-2.841,2.844c-0.178,0.176-0.178,0.461,0,0.637c0.178,0.178,0.459,0.178,0.637,0l2.844-2.841l2.844,2.841c0.178,0.178,0.459,0.178,0.637,0c0.178-0.176,0.178-0.461,0-0.637L10.824,10z"></path>
                    </svg>
                </div>
              </button>
            </h2>
            <div id="accordion-collapse-body-3" class={if *collapsed == false {"hidden"} else {"block"}} ariaLabelledby="accordion-collapse-heading-3">
              <div class="p-5 font-light border justify-self-center mx-auto grid grid-cols-4 gap-y-8 border-t-0 border-gray-200 dark:border-gray-700">
                //  <Image />
                //  <Image />
                //  <Image />
                //  <Image />
                //  <Image />

              </div>
            </div>
          </div>
    }
}
