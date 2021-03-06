use log::*;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/highlight.js")]
extern "C" {
  pub fn highlight();
}

#[function_component(App)]
pub fn app() -> Html {

    {
      use_effect(move || {
        info!("rendered!");
          // Make a call to DOM API after component is rendered
          highlight();

            // Perform the cleanup
            || highlight()
        });
    }
        
    
    html! {
    <>
        // <div class="flex h-screen overflow-hidden bg-white rounded-lg">
        //     <div class="hidden md:flex md:flex-shrink-0">
        //         <div class="flex flex-col w-64">
        //         <div class="
        //         flex flex-col flex-grow
        //         pt-5
        //         overflow-y-auto
        //         bg-white
        //         border-r border-gray-50
        //     ">
        //             <div class="flex flex-col items-center flex-shrink-0 px-4">
        //             <a href="./index.html" class="px-8 text-left focus:outline-none">
        //                 <h2 class="
        //             block
        //             p-2
        //             text-xl
        //             font-medium
        //             tracking-tighter
        //             text-gray-900
        //             transition
        //             duration-500
        //             ease-in-out
        //             transform
        //             cursor-pointer
        //             hover:text-gray-900
        //             "> {"wickedblocks"} </h2>
        //             </a>
        //             <button class="hidden rounded-lg focus:outline-none focus:shadow-outline">
        //                 <svg fill="currentColor" viewBox="0 0 20 20" class="w-6 h-6">
        //                 <path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM9 15a1 1 0 011-1h6a1 1 0 110 2h-6a1 1 0 01-1-1z" clip-rule="evenodd"></path>
        //                 <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
        //                 </svg>
        //             </button>
        //             </div>
        //             <div class="flex flex-col flex-grow px-4 mt-5">
        //             <nav class="flex-1 space-y-1 bg-white">
        //                 <ul>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 bg-gray-50
        //                 focus:shadow-outline
        //                 " white="" href="#">
        //                     <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
        //                     </svg>
        //                     <span class="ml-4"> {"Overview"}</span>
        //                     </a>
        //                 </li>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 focus:shadow-outline
        //                 hover:bg-gray-50
        //                 " href="#">
        //                     <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
        //                     </svg>
        //                     <span class="ml-4">{"Chat"}</span>
        //                     </a>
        //                 </li>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 focus:shadow-outline
        //                 hover:bg-gray-50
        //                 " href="#">
        //                     <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
        //                     </svg>
        //                     <span class="ml-4">{"User"}</span>
        //                     </a>
        //                 </li>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 focus:shadow-outline
        //                 hover:bg-gray-50
        //                 " href="#"><svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
        //                     </svg>
        //                     <span class="ml-4">{"Settings"}</span></a>
        //                 </li>
        //                 </ul>
        //                 <p class="px-4 pt-4 font-medium text-gray-900 uppercase"> {"Shortcuts"} </p>
        //                 <ul>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 focus:shadow-outline
        //                 hover:bg-gray-50
        //                 " white="" href="#">
        //                     <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path>
        //                     </svg>
        //                     <span class="ml-4"> {"Tasks"}</span>
        //                     </a>
        //                 </li>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 focus:shadow-outline
        //                 hover:bg-gray-50
        //                 " white="" href="#">
        //                     <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
        //                     </svg>
        //                     <span class="ml-4"> {"Reports"}</span>
        //                     </a>
        //                 </li>
        //                 <li>
        //                     <a class="
        //                 inline-flex
        //                 items-center
        //                 w-full
        //                 px-4
        //                 py-2
        //                 mt-1
        //                 text-base text-gray-900
        //                 transition
        //                 duration-500
        //                 ease-in-out
        //                 transform
        //                 rounded-lg
        //                 focus:shadow-outline
        //                 hover:bg-gray-50
        //                 " white="" href="#">
        //                     <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 3.055A9.001 9.001 0 1020.945 13H11V3.055z"></path>
        //                         <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.488 9H15V3.512A9.025 9.025 0 0120.488 9z"></path>
        //                     </svg>
        //                     <span class="ml-4"> {"Dashboard"}</span>
        //                     </a>
        //                 </li>
        //                 </ul>
        //             </nav>
        //             </div>
        //             <div class="flex flex-shrink-0 p-4 px-4 bg-gray-50">
        //             <a href="#" class="flex-shrink-0 block w-full group">
        //                 <div class="flex items-center">
        //                 <div>
        //                     <img class="inline-block rounded-full h-9 w-9" src="./images/wickedlabslogo.jpg" alt=""/>
        //                 </div>
        //                 <div class="ml-3">
        //                     <p class="text-sm font-medium text-gray-900">{"Wicked Labs"}</p>
        //                 </div>
        //                 </div>
        //             </a>
        //             </div>
        //         </div>
        //         </div>
        //     </div>
        //     <div class="flex flex-col flex-1 w-0 overflow-hidden">
        //         <main class="relative flex-1 overflow-y-auto focus:outline-none">
        //         <div class="py-6">
        //             <div class="px-4 mx-auto max-w-7xl sm:px-6 md:px-8">
        //             <h1 class="text-lg text-neutral-600">{" Here is where you put your stuff "}</h1>
        //             </div>
        //             <div class="px-4 mx-auto max-w-7xl sm:px-6 md:px-8">
        //             // <!-- Put your content here -->
        //             <pre>
        //                 <code class="json">
        //                     {r#"
        //                     {
        //                         "FirstName": "John",
        //                         "LastName": "Doe",
        //                         "Age": 43,
        //                         "Address": {
        //                             "Street": "Downing Street 10",
        //                             "City": "London",
        //                             "Country": "Great Britain"
        //                         },
        //                         "PhoneNumbers": [
        //                             "+44 1234567",
        //                             "+44 2345678"
        //                         ]
        //                     }
        //                     "#}
        //                 </code>
        //             </pre>

        //             <pre>
        //                 <code class="javascript">
        //                     {r#"
        //                     const value = "value";
        //                     const number = 1;
        //                     if (value) {
        //                         console.log(number)
        //                     } else {
        //                         console.log(value)
        //                     }
        //                     "#}
        //                 </code>
        //             </pre>
        //             <div class="py-4">
        //                 <div class="rounded-lg bg-gray-50 h-96"></div>
        //             </div>
        //             // <!-- Do not cross the closing tag underneath this coment-->
        //             </div>
        //         </div>
        //         </main>
        //     </div>
        // </div>


        // Navbar
        <div class="container w-full">
            <div x-data="{ open: false }" class="
          flex flex-col
          max-w-screen-xl
          p-5
          mx-auto
          md:items-center md:justify-between md:flex-row md:px-6
          lg:px-8
        ">
              <div class="flex flex-row items-center justify-between lg:justify-start">
                <a href="#" class="
              text-lg
              font-bold
              tracking-tighter
              text-blue-400
              transition
              duration-500
              ease-in-out
              transform
              tracking-relaxed
              lg:pr-8
            "> {"Domain Doc"} </a>
                <button class="rounded-lg md:hidden focus:outline-none focus:shadow-outline">
                  <svg fill="currentColor" viewBox="0 0 20 20" class="w-8 h-8">
                    <path x-show="!open" fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM9 15a1 1 0 011-1h6a1 1 0 110 2h-6a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                    <path x-show="open" fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" style="display: none"></path>
                  </svg>
                </button>
              </div>
              <nav class="flex-col flex-grow hidden md:flex md:justify-end md:flex-row">
                <ul class="space-y-2 list-none lg:space-y-0 lg:items-center lg:inline-flex">
                  <li>
                    <a href="#" class="
                  px-2
                  lg:px-6
                  py-6
                  text-sm
                  border-b-2 border-transparent
                  hover:border-blue-600
                  leading-[22px]
                  md:px-3
                  text-gray-500
                  hover:text-blue-500
                "> {"Auth Doc"}
                    </a>
                  </li>
                  <li>
                    <a href="#" class="
                  px-2
                  lg:px-6
                  py-6
                  text-sm
                  border-b-2 border-transparent
                  hover:border-blue-600
                  leading-[22px]
                  md:px-3
                  text-gray-500
                  hover:text-blue-500
                "> {"Management API Doc"} </a>
                  </li>
                  <li>
                    <a href="#" class="
                  px-2
                  lg:px-6
                  py-6
                  text-sm
                  border-b-2 border-transparent
                  hover:border-blue-600
                  leading-[22px]
                  md:px-3
                  text-gray-500
                  hover:text-blue-500
                "> {"User Guide"} 
                    </a>
                  </li>
                </ul>
              </nav>
            </div>
          </div>


        <div class="grid grid-cols-5 border-t-4 border-gray-300">
        <div class="w-full px-6 text-xl text-gray-800 leading-normal bg-gray-300">
        <div class="block lg:hidden sticky inset-0">
            <button id="menu-toggle" class="flex w-full justify-end px-3 py-3 bg-white lg:bg-transparent border rounded border-gray-600 hover:border-yellow-600 appearance-none focus:outline-none">
                <svg class="fill-current h-3 float-right" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" />
                </svg>
            </button>
        </div>
        <div class="w-full sticky inset-0 hidden max-h-64 lg:h-auto overflow-x-hidden overflow-y-auto lg:overflow-y-hidden lg:block mt-0 my-2 lg:my-0 border border-gray-400 lg:border-transparent bg-white shadow lg:shadow-none lg:bg-transparent z-20" style="top:6em;" id="menu-content">
            <ul class="list-reset py-2 md:py-0">
                <li class="py-1 md:my-2 hover:bg-yellow-100 lg:hover:bg-transparent border-l-4 border-transparent font-bold border-yellow-600">
                    <a href="#section1" class="block pl-4 align-middle text-gray-700 no-underline hover:text-yellow-600">
                        <span class="pb-1 md:pb-0 text-sm">{"Clients"}</span>
                    </a>
                </li>
                <li class="py-1 md:my-2 hover:bg-yellow-100 lg:hover:bg-transparent border-l-4 border-transparent">
                    <a href="#section2" class="block pl-4 align-middle text-gray-700 no-underline hover:text-yellow-600">
                        <span class="pb-1 md:pb-0 text-sm">{"Resource Servers"}</span>
                    </a>
                </li>
                <li class="py-1 md:my-2 hover:bg-yellow-100 lg:hover:bg-transparent border-l-4 border-transparent">
                    <a href="#section3" class="block pl-4 align-middle text-gray-700 no-underline hover:text-yellow-600">
                        <span class="pb-1 md:pb-0 text-sm">{"Roles"}</span>
                    </a>
                </li>
                <li class="py-1 md:my-2 hover:bg-yellow-100 lg:hover:bg-transparent border-l-4 border-transparent">
                    <a href="#section4" class="block pl-4 align-middle text-gray-700 no-underline hover:text-yellow-600">
                        <span class="pb-1 md:pb-0 text-sm">{"User"}</span>
                    </a>
                </li>

                <li class="py-1 md:my-2 hover:bg-yellow-100 lg:hover:bg-transparent border-l-4 border-transparent">
                    <a href="#section5" class="block pl-4 align-middle text-gray-700 no-underline hover:text-yellow-600">
                        <span class="pb-1 md:pb-0 text-sm">{"Tenants"}</span>
                    </a>
                </li>
            </ul>
        </div>
    </div>
            <div class="col-span-4 divide-y-4 divide-slate-400/25">
            // Introduction section
              <div class="grid grid-cols-2">
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="my-2 text-2xl font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Introduction"}</h4>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"This document describes the Domain Management API, which is meant to be used by back-end servers or trusted parties performing administrative tasks. Generally speaking, anything that can be done through the Domain dashboard (and more) can also be done through this API.
                        This API is separate from the publicly accessible Auth0 Authentication API, which is meant to be used by front-ends and untrusted parties.
                        When using the code samples, requests should be sent with a Content-Type of "}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500 bg-gray-300 javascript">{"application/json."}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{" All endpoints accept a maximum payload size of 1 megabyte."}</span>
                        </p>                        
                        </div>
                      </div>
                </div>
              </div>

                    
              <div class="relative items-center w-full mx-auto bg-gray-500">
                
              </div>

              </div>

              // Register Section
              <div class="grid grid-cols-2">
                  <div class="relative items-center w-full mx-auto">
                    <div class="grid grid-cols-1 ">
                      <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                        <div class="p-4">
                          <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Create Account"}</h4>
                          <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
                          <span class="mt-3 text-base leading-relaxed text-gray-500">{"/register"}</span>
                          <p class="mt-3 text-base leading-relaxed text-gray-500">
                          <span class="mt-3 text-base leading-relaxed text-gray-500">{"Given a user's credentials, and a "}</span>
                          <span class="mt-3 text-base leading-relaxed text-gray-500 bg-gray-300 javascript">{"connection"}</span>
                          <span class="mt-3 text-base leading-relaxed text-gray-500">{", this endpoint will create a new user using active authentication. This endpoint only works for database connections."}</span>
                          </p>
                          </div>
                        </div>
                      </div>
                    </div>

                    
                    <div class="relative items-center w-full mx-auto bg-gray-500">
                    // Request body section
                      <div>
                        <div class="p-4">
                          <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                          <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/x-www-form-urlencoded "}</h4>
                        </div>
                        <pre>
                          <code class="json text-xs">
                            {r#"
        {
          "email": "blabla@email.com",
          "password": "password minimal 8, huruf kecil, huruf besar, nomer",
          "name": "username",
          "account_type": "personal / businness",
          "region": "us / au / jp / eu",
          "tenant_name": "first line of domain for users",
        }                       
                            "#}
                            </code>
                        </pre>        
                      </div>
                      
                      // Req Headers Section
                      <div>
                        <div class="p-4">
                          <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers"}</h4>
                        </div>
                        <pre>
                          <code class="json text-lg">
                            {r#"



        No Headers Required
                            
                            

                            "#}
                            </code>
                        </pre>        
                      </div>
                      
                      // Response section
                      <div>
                        <div class="p-4">
                          <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                        </div>
                        <pre>
                          <code class="json text-xs">
                            {r#"

        Status Code 201 - Created

        {
          "message": "You are sucessfully registered.",
          "data": "registered email"
        }

        Status Code 400 - Bad Request(validate field email dan password)

        {
          "message": "validation error",
          "data": ""
        }

        Status Code 400 - Bad Request(validate unique email)

        {
            "message": "Email is already used.",
            "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }

                            "#}
                            </code>
                        </pre>        
                      </div>

                    </div>
              </div>

              // Login Section
              <div class="grid grid-cols-2">
                  <div class="relative items-center w-full mx-auto">
                    <div class="grid grid-cols-1 ">
                      <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                        <div class="p-4">
                          <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Login Account"}</h4>
                          <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
                          <span class="mt-3 text-base leading-relaxed text-gray-500">{"/login"}</span>
                          <p class="mt-3 text-base leading-relaxed text-gray-500">
                          <span class="mt-3 text-base leading-relaxed text-gray-500">{"Given the social provider's Access Token and the connection, this endpoint will authenticate the user with the provider and return a JSON with the Access Token."}</span>
                          </p>
                          </div>
                        </div>
                      </div>
                    </div>

                    
                    <div class="relative items-center w-full mx-auto bg-gray-500">
                      // Request body section
                      <div>
                        <div class="p-4">
                          <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                          <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/x-www-form-urlencoded "}</h4>
                        </div>
                        <pre>
                          <code class="json text-xs">
                            {r#"
        {
          "email": "blabla@email.com",
          "password": "password minimal 8, huruf kecil, huruf besar, nomer"
        }             
                            "#}
                            </code>
                        </pre>        
                      </div>
                      
                      // Req Headers Section
                      <div>
                        <div class="p-4">
                          <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers"}</h4>
                        </div>
                        <pre>
                          <code class="json text-lg">
                            {r#"



        No Headers Required
                            
                            

                            "#}
                            </code>
                        </pre>        
                      </div>
                      
                      // Response section
                      <div>
                        <div class="p-4">
                          <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                        </div>
                        <pre>
                          <code class="json text-xs">
                            {r#"

        Status Code 200 - Ok

        {
          "email": "blabla@email.com",
          "username": "username",
          "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI2MjBlMGZmMzk5YzgwMDhkZjY4Yzc0OTIiLCJleHAiOjE2NDUxNzUxNTUsImlhdCI6MTY0NTA4ODc1NSwiZW1haWwiOiJoZXlrYWxsQGdtYWlsLmNvbSIsInRlbmFudCI6IkRvbWFpbiIsInNjb3BlcyI6InJlYWQ6Y2xpZW50X2dyYW50cyBjcmVhdGU6Y2xpZW50X2dyYW50cyBkZWxldGU6Y2xpZW50X2dyYW50cyB1cGRhdGU6Y2xpZW50X2dyYW50cyByZWFkOnVzZXJzIHVwZGF0ZTp1c2VycyBkZWxldGU6dXNlcnMgY3JlYXRlOnVzZXJzIHJlYWQ6dXNlcnNfYXBwX21ldGFkYXRhIHVwZGF0ZTp1c2Vyc19hcHBfbWV0YWRhdGEgZGVsZXRlOnVzZXJzX2FwcF9tZXRhZGF0YSBjcmVhdGU6dXNlcnNfYXBwX21ldGFkYXRhIHJlYWQ6dXNlcl9jdXN0b21fYmxvY2tzIGNyZWF0ZTp1c2VyX2N1c3RvbV9ibG9ja3MgZGVsZXRlOnVzZXJfY3VzdG9tX2Jsb2NrcyBjcmVhdGU6dXNlcl90aWNrZXRzIHJlYWQ6Y2xpZW50cyB1cGRhdGU6Y2xpZW50cyBkZWxldGU6Y2xpZW50cyBjcmVhdGU6Y2xpZW50cyByZWFkOmNsaWVudF9rZXlzIHVwZGF0ZTpjbGllbnRfa2V5cyBkZWxldGU6Y2xpZW50X2tleXMgY3JlYXRlOmNsaWVudF9rZXlzIHJlYWQ6Y29ubmVjdGlvbnMgdXBkYXRlOmNvbm5lY3Rpb25zIGRlbGV0ZTpjb25uZWN0aW9ucyBjcmVhdGU6Y29ubmVjdGlvbnMgcmVhZDpyZXNvdXJjZV9zZXJ2ZXJzIHVwZGF0ZTpyZXNvdXJjZV9zZXJ2ZXJzIGRlbGV0ZTpyZXNvdXJjZV9zZXJ2ZXJzIGNyZWF0ZTpyZXNvdXJjZV9zZXJ2ZXJzIHJlYWQ6ZGV2aWNlX2NyZWRlbnRpYWxzIHVwZGF0ZTpkZXZpY2VfY3JlZGVudGlhbHMgZGVsZXRlOmRldmljZV9jcmVkZW50aWFscyBjcmVhdGU6ZGV2aWNlX2NyZWRlbnRpYWxzIHJlYWQ6cnVsZXMgdXBkYXRlOnJ1bGVzIGRlbGV0ZTpydWxlcyBjcmVhdGU6cnVsZXMgcmVhZDpydWxlc19jb25maWdzIHVwZGF0ZTpydWxlc19jb25maWdzIGRlbGV0ZTpydWxlc19jb25maWdzIHJlYWQ6aG9va3MgdXBkYXRlOmhvb2tzIGRlbGV0ZTpob29rcyBjcmVhdGU6aG9va3MgcmVhZDphY3Rpb25zIHVwZGF0ZTphY3Rpb25zIGRlbGV0ZTphY3Rpb25zIGNyZWF0ZTphY3Rpb25zIHJlYWQ6ZW1haWxfcHJvdmlkZXIgdXBkYXRlOmVtYWlsX3Byb3ZpZGVyIGRlbGV0ZTplbWFpbF9wcm92aWRlciBjcmVhdGU6ZW1haWxfcHJvdmlkZXIgYmxhY2tsaXN0OnRva2VucyByZWFkOnN0YXRzIHJlYWQ6aW5zaWdodHMgcmVhZDp0ZW5hbnRfc2V0dGluZ3MgdXBkYXRlOnRlbmFudF9zZXR0aW5ncyByZWFkOmxvZ3MgcmVhZDpsb2dzX3VzZXJzIHJlYWQ6c2hpZWxkcyBjcmVhdGU6c2hpZWxkcyB1cGRhdGU6c2hpZWxkcyBkZWxldGU6c2hpZWxkcyByZWFkOmFub21hbHlfYmxvY2tzIGRlbGV0ZTphbm9tYWx5X2Jsb2NrcyB1cGRhdGU6dHJpZ2dlcnMgcmVhZDp0cmlnZ2VycyByZWFkOmdyYW50cyBkZWxldGU6Z3JhbnRzIHJlYWQ6Z3VhcmRpYW5fZmFjdG9ycyB1cGRhdGU6Z3VhcmRpYW5fZmFjdG9ycyByZWFkOmd1YXJkaWFuX2Vucm9sbG1lbnRzIGRlbGV0ZTpndWFyZGlhbl9lbnJvbGxtZW50cyBjcmVhdGU6Z3VhcmRpYW5fZW5yb2xsbWVudF90aWNrZXRzIHJlYWQ6dXNlcl9pZHBfdG9rZW5zIGNyZWF0ZTpwYXNzd29yZHNfY2hlY2tpbmdfam9iIGRlbGV0ZTpwYXNzd29yZHNfY2hlY2tpbmdfam9iIHJlYWQ6Y3VzdG9tX2RvbWFpbnMgZGVsZXRlOmN1c3RvbV9kb21haW5zIGNyZWF0ZTpjdXN0b21fZG9tYWlucyB1cGRhdGU6Y3VzdG9tX2RvbWFpbnMgcmVhZDplbWFpbF90ZW1wbGF0ZXMgY3JlYXRlOmVtYWlsX3RlbXBsYXRlcyB1cGRhdGU6ZW1haWxfdGVtcGxhdGVzIHJlYWQ6bWZhX3BvbGljaWVzIHVwZGF0ZTptZmFfcG9saWNpZXMgcmVhZDpyb2xlcyBjcmVhdGU6cm9sZXMgZGVsZXRlOnJvbGVzIHVwZGF0ZTpyb2xlcyByZWFkOnByb21wdHMgdXBkYXRlOnByb21wdHMgcmVhZDpicmFuZGluZyB1cGRhdGU6YnJhbmRpbmcgZGVsZXRlOmJyYW5kaW5nIHJlYWQ6bG9nX3N0cmVhbXMgY3JlYXRlOmxvZ19zdHJlYW1zIGRlbGV0ZTpsb2dfc3RyZWFtcyB1cGRhdGU6bG9nX3N0cmVhbXMgY3JlYXRlOnNpZ25pbmdfa2V5cyByZWFkOnNpZ25pbmdfa2V5cyB1cGRhdGU6c2lnbmluZ19rZXlzIHJlYWQ6bGltaXRzIHVwZGF0ZTpsaW1pdHMgY3JlYXRlOnJvbGVfbWVtYmVycyByZWFkOnJvbGVfbWVtYmVycyBkZWxldGU6cm9sZV9tZW1iZXJzIHJlYWQ6ZW50aXRsZW1lbnRzIHJlYWQ6YXR0YWNrX3Byb3RlY3Rpb24gdXBkYXRlOmF0dGFja19wcm90ZWN0aW9uIHJlYWQ6b3JnYW5pemF0aW9ucyB1cGRhdGU6b3JnYW5pemF0aW9ucyBjcmVhdGU6b3JnYW5pemF0aW9ucyBkZWxldGU6b3JnYW5pemF0aW9ucyBjcmVhdGU6b3JnYW5pemF0aW9uX21lbWJlcnMgcmVhZDpvcmdhbml6YXRpb25fbWVtYmVycyBkZWxldGU6b3JnYW5pemF0aW9uX21lbWJlcnMgY3JlYXRlOm9yZ2FuaXphdGlvbl9jb25uZWN0aW9ucyByZWFkOm9yZ2FuaXphdGlvbl9jb25uZWN0aW9ucyB1cGRhdGU6b3JnYW5pemF0aW9uX2Nvbm5lY3Rpb25zIGRlbGV0ZTpvcmdhbml6YXRpb25fY29ubmVjdGlvbnMgY3JlYXRlOm9yZ2FuaXphdGlvbl9tZW1iZXJfcm9sZXMgcmVhZDpvcmdhbml6YXRpb25fbWVtYmVyX3JvbGVzIGRlbGV0ZTpvcmdhbml6YXRpb25fbWVtYmVyX3JvbGVzIGNyZWF0ZTpvcmdhbml6YXRpb25faW52aXRhdGlvbnMgcmVhZDpvcmdhbml6YXRpb25faW52aXRhdGlvbnMgZGVsZXRlOm9yZ2FuaXphdGlvbl9pbnZpdGF0aW9ucyJ9.P0IhQqmzWe5uQ1eQxQTIX2xESNaGvcCeN1qwJs3247g"
        }

        Status Code 401 - Unauthorized

        {
          "message": "Wrong username or password, please try again",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }

                            "#}
                            </code>
                        </pre>        
                      </div>

                    </div>
              </div>
              
              // Get Client Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get clients"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/clients"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve clients (applications and SSO integrations) matching provided filters. A list of fields to include or exclude may also be specified."}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        {"Note:"}
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        {"Note:"}
                        </p>  
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required



                        "#}
                        </code>
                    </pre>          
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"


                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Succesfully get all Applications",
          "data": [
              {
                  "id": "AaiyAPdpYdesoKnqjj8HJqRn4T5titww",
                  "name": "name app",
                  "client_id": "client id app",
                  "app_type": "app type",
                  "logo_url": "url logo"
              },
              {
                  "id": "kmzWay87dsdDSdsadffsSFGWWWfsaggS",
                  "name": "name app",
                  "client_id": "client id app",
                  "app_type": "app type",
                  "logo_url": "url logo"
              },
          ]
        }
                  
        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }

                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Post Client Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Create a client"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/clients"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Create a new client (application)."}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        {"Note: We recommend leaving the `client_secret` parameter unspecified to allow the generation of a safe secret."}
                        </p>
                        <p class="mt-3 bg-yellow-500 text-base p-4 leading-relaxed text-gray-500">
                        {"SSO Integrations created via this endpoint will accept login requests and share user profile information."}
                        </p>  
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                      <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/x-www-form-urlencoded "}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"
        {
            "name": "name app",
            "app_type": "type of app: default is 'custom app'"
        }
                        "#}
                        </code>
                    </pre>          
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 201 - Created

        {
        "message": "Successfullly create Application",
        "data": {
            "id": 1,
            "name": "name of application",
            "client_id": "auto generated client id",
            "app_type": "custom app",
            "domain": "concat from tenant_name and tenant region",
            "client_secret": "auto generated client secret",
            "description": "description of application",
            "logo_url": "url logo of application",
            "token_auth_method": "default 'Post'",
            "app_login_url": "url app login",
            "callback_url": "url callback",
            "logout_url": "url logout",
            "web_origin": "url web origin",
            "cors": "url cors",
            "id_token_exp": "default integer",
            "reuse_interval": "default integer",
            "abs_lifetime": "default integer",
            "inactivity_lifetime": "default integer",
            "tenant_id": id tenant
          }
        }
                                          
        Status Code 400 - Bad Request

        {
          "message": "Parse error",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }

                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>


              // Get Detail Client Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get Details a client"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/clients/{id}"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve client details. A list of fields to include or exclude may also be specified."}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        {"Note: "}
                        </p> 
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required
                        
                        

                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // request parameter section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        endpoint: /api/v2/clients/{id}
        example: /api/v2/clients/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        "id": "id of app"


                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Successfullly create Application",
          "data": {
              "id": "AaiyAPdpYdesoKnqjj8HJqRn4T5titww",
              "name": "name of application",
              "client_id": "auto generated client id",
              "app_type": "custom app",
              "domain": "concat from tenant_name and tenant region",
              "client_secret": "auto generated client secret",
              "description": "description of application",
              "logo_url": "url logo of application",
              "token_auth_method": "default 'Post'",
              "app_login_url": "url app login",
              "callback_url": "url callback",
              "logout_url": "url logout",
              "web_origin": "url web origin",
              "cors": "url cors",
              "id_token_exp": "default integer",
              "reuse_interval": "default integer",
              "abs_lifetime": "default integer",
              "inactivity_lifetime": "default integer",
              "tenant_id": id tenant
            }
        }
                  
        Status Code 403 - Forbidden

        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 404 - Not Found

        {
          "message": "Application not found",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Patch Update a Client Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Update a client"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-yellow-400 rounded-full">{"PATCH"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/clients/{id}"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Update details of a client."}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        {"Note: The `client_secret` and `signing_key` attributes can only be updated with the `update:client_keys` scope."}
                        </p> 
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                      <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"
        {
          "name": "name app",
          "client_id": "client id app",
          "app_type": "type app",
          "domain": "domain app",
          "client_secret": "updated client secret app",
          "description": "description app",
          "logo_url": "url logo app",
          "token_auth_method": "token auth method app",
          "app_login_url": "url app login",
          "callback_url": "url callback app",
          "logout_url": "url logout app",
          "web_origin": "url web origin app",
          "cors": "url cors app",
          "id_token_exp": integer,
          "reuse_interval": integer,
          "abs_lifetime": integer,
          "inactivity_lifetime": integer,
        }
                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // request parameter section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        endpoint: /api/v2/clients/{id}
        example: /api/v2/clients/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        "id": "id of app"


                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Successfully update a Application Client",
          "data": {
              "id": "AaiyAPdpYdesoKnqjj8HJqRn4T5titww",
              "name": "name of application",
              "client_id": "auto generated client id",
              "app_type": "custom app",
              "domain": "concat from tenant_name and tenant region",
              "client_secret": "auto generated client secret",
              "description": "description of application",
              "logo_url": "url logo of application",
              "token_auth_method": "default 'Post'",
              "app_login_url": "url app login",
              "callback_url": "url callback",
              "logout_url": "url logout",
              "web_origin": "url web origin",
              "cors": "url cors",
              "id_token_exp": "default integer",
              "reuse_interval": "default integer",
              "abs_lifetime": "default integer",
              "inactivity_lifetime": "default integer",
              "tenant_id": id tenant
            }
        }
                  
        Status Code 403 - Forbidden

        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 404 - Not Found

        {
          "message": "Application not found",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Delete a Client Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Delete a client"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-red-400 rounded-full">{"DELETE"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/clients/{id}"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Delete a client and related configuration (rules, connections, etc)."}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required
                        
                        

                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // request parameter section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        endpoint: /api/v2/clients/{id}
        example: /api/v2/clients/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        "id": "id of app"

                                
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Successfully delete the Application",
          "data": {
              "id": "AaiyAPdpYdesoKnqjj8HJqRn4T5titww",
              "name": "name of application",
              "client_id": "auto generated client id",
              "app_type": "custom app",
              "domain": "concat from tenant_name and tenant region",
              "client_secret": "auto generated client secret",
              "description": "description of application",
              "logo_url": "url logo of application",
              "token_auth_method": "default 'Post'",
              "app_login_url": "url app login",
              "callback_url": "url callback",
              "logout_url": "url logout",
              "web_origin": "url web origin",
              "cors": "url cors",
              "id_token_exp": "default integer",
              "reuse_interval": "default integer",
              "abs_lifetime": "default integer",
              "inactivity_lifetime": "default integer",
              "tenant_id": id tenant
            }
        }
                  
        Status Code 403 - Forbidden

        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 404 - Not Found

        {
          "message": "Application not found",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Get Resource Servers Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get resource servers"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/resource-servers"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve "}</span>
                        <span class="mt-3 text-base leading-relaxed text-violet-500">{"APIs"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{" (also known as resource servers) that you can consume from your authorized applications."}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required
                        
                        

                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Api fetched",
          "data": [
              {
                "id": "api id",
                "name": "name API",
                "api_type": "type API",
                "identifier": "url identifier API"
              },
              {
                "id": "api id",
                "name": "name API",
                "api_type": "type API",
                "identifier": "url identifier API"
              }
            ]
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Create Resource Servers Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Create a resource server"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/resource-server"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Create a new API (also known as a resource server)."}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                      <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"
        {
          "name": "name API",
          "identifier": "url identifier API",
          "sign_algorithm": "signature algorithm for API"
        }
                        "#}
                        </code>
                    </pre>          
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 201 - Created

        {
          "message": "Succesfully create api",
          "data": ""
        }
                  
        Status Code 400 - Bad Request

        {
          "message": "Parse error",
          "data": ""
        }

        Status Code 403 - Forbidden

        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }

                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Get Detail a Resource Server Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get Details a resource server"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/resource-server/{id}"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve an "}</span>
                        <span class="mt-3 text-base leading-relaxed text-violet-500">{"APIs"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{" (also known as resource server)."}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required
                        
                        

                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


                        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // request parameter section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        endpoint: /api/v2/resource-server/{id}
        example: /api/v2/resource-server/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        "id": "id of API"

        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Api loaded",
          "data": {
            "id": "api id",
            "name": "name api",
            "api_id": "auto generated id",
            "api_type": "Custom API",
            "identifier": "url identifier",
            "token_exp": integer,
            "token_exp_browser": integer,
            "sign_algorithm": "algorithm signing",
            "rbac": boolean,
            "permission_acc_token": boolean,
            "allow_skip_user": boolean,
            "allow_off_acc": boolean,
            "tenant_id": 1
          }
        }
                  
        Status Code 403 - Forbidden
        
        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 404 - Not Found
        
        {
          "message": "Api not found",
          "data": ""
        }

        Status Code 500 - Internal Server Error
        
        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Patch Update a Resource Server
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Update a resource server"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-yellow-400 rounded-full">{"PATCH"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/resource-servers/{id}"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Update an existing "}</span>
                        <span class="mt-3 text-base leading-relaxed text-violet-500">{"API"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{" (also known as resource server)."}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                      <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"
        {
          "name": "name app",
          "token_exp": integer,
          "token_exp_browser": integer,
          "rbac": boolean,
          "permission_acc_token": boolean,
          "allow_skip_user": boolean,
          "allow_off_acc": boolean,
        }
                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // request parameter section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        endpoint: /api/v2/resource-servers/{id}
        example: /api/v2/resource-servers/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        "id": "id of api"

        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Successfully update Api",
            "data": {
              "id": 1,
              "name": "name api",
              "api_id": "auto generated id",
              "api_type": "Custom API",
              "identifier": "url identifier",
              "token_exp": integer,
              "token_exp_browser": integer,
              "sign_algorithm": "algorithm signing",
              "rbac": boolean,
              "permission_acc_token": boolean,
              "allow_skip_user": boolean,
              "allow_off_acc": boolean,
              "tenant_id": 1
            }
        }

        Status Code 400 - Bad Request
        
        {
          "message": "Parse error",
          "data": ""
        }

        Status Code 403 - Forbidden
        
        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 404 - Not Found
        
        {
          "message": "Api not found",
          "data": ""
        }

        Status Code 500 - Internal Server Error
        
        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // Delete a resourse server Section
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Delete a resource server"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-red-400 rounded-full">{"DELETE"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/resource-servers/{id}"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Delete an existing API (also known as a resource server)."}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required
                        
                        

                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // request parameter section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        endpoint: /api/v2/resource-servers/{id}
        example: /api/v2/resource-servers/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        "id": "id of api"

        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Api deleted",
          "data": ""
        }

                  
        Status Code 403 - Forbidden
        
        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 403 - Forbidden
        
        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 404 - Not Found
        
        {
          "message": "Api not found",
          "data": ""
        }

        Status Code 500 - Internal Server Error
        
        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

              // get All Users
              <div class="grid grid-cols-2">
                // description
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
                      <div class="p-4">
                        <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"List or Search Users"}</h4>
                        <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users"}</span>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve details of users. It is possible to: "}</span>
                        </p>
                        <br/>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"- Specify a search criteria for users"}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"- Sort the users to be returned"}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"- Select the fields to be returned"}</span>
                        </p>
                        <p class="mt-3 text-base leading-relaxed text-gray-500">
                        <span class="mt-3 text-base leading-relaxed text-gray-500">{"- Specify the number of users to retrieve per page and the page index"}</span>
                        </p>
                        </div>
                      </div>
                    </div>
                </div>
                
                // response section
                <div class="relative items-center w-full mx-auto bg-gray-500">
                  // Request body section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-lg">
                        {r#"



        No Request Body Required
                        
                        

                        "#}
                        </code>
                    </pre>      
                  </div>
                  
                  // Request Headers Section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"


        "access_token": "value from field 'token' on method POST /login"

                        
                        "#}
                        </code>
                    </pre>        
                  </div>
                  
                  // Response section
                  <div>
                    <div class="p-4">
                      <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                    </div>
                    <pre>
                      <code class="json text-xs">
                        {r#"

        Status Code 200 - Ok

        {
          "message": "Ok",
          "data": [
              {
                "id": 1,
                "email": "email user",
                "name": "name user",
                "connection": "connection user: default -> Username-Password-Authentication",
                "login_counter": integer,
                "latest_login": "timestamp now"
              },
              {
                "id": 2,
                "email": "email user",
                "name": "name user",
                "connection": "connection user: default -> Username-Password-Authentication",
                "login_counter": integer,
                "latest_login": "timestamp now"
              },
            ]
        }

        Status Code 403 - Forbidden

        {
          "message": "You dont have access to this data",
          "data": ""
        }

        Status Code 500 - Internal Server Error

        {
          "message": "Internal Server Error",
          "data": ""
        }
                        "#}
                        </code>
                    </pre>        
                  </div>

                </div>
              </div>

        //       //create A User
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Create a User"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Create a new user for a given database or passwordless connection."}</span>
        //                 </p>
        //                 <br/>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Note: connection is required but other parameters such as email and password are dependent upon the type of connection."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //               <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"
        // {
        //   "email": "email user",
        //   "password": "password user",
        //   "connection": "connection user: default -> Username-Password-Authentication",
        // }
        //                 "#}
        //                 </code>
        //             </pre>          
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 201 - Created

        // {
        //   "message": "Users created",
        //   "data": {
        //     "created_at": "timestamp",
        //     "email": "email user",
        //     "identities": [
        //       {
        //         "connection": "connection user: default -> Username-Password-Authentication",
        //         "user_id": "user id",
        //         "provider": "default -> domain",
        //         "is_social": boolean
        //       }
        //     ],
        //     "name": "name user from email",
        //     "picture": "url picture user",
        //     "updated_at": "timestamp",
        //     "user_id": "concat from field provider and user_id",
        //     "blocked": boolean,
        //     "blocked_for": [],
        //     "guardian_authenticators": []
        //   }
        // }

        // Status Code 400 - Bad Request

        // {
        //   "message": "Parse error",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden

        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error

        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Get Detail Users Section
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get a User"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve user details. A list of fields to include or exclude may also be specified."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        // "id": "id of User"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "Ok",
        //   "data": {
        //     "created_at": "timestamp",
        //     "email": "email user",
        //     "identities": [
        //       {
        //         "connection": "connection user: default -> Username-Password-Authentication",
        //         "user_id": "user id",
        //         "provider": "default -> domain",
        //         "is_social": boolean
        //       }
        //     ],
        //     "name": "name from email",
        //     "picture": "url picture of user",
        //     "updated_at": "timestamp",
        //     "user_id": "concat from field provider and user_id",
        //     "blocked": boolean,
        //     "blocked_for": [],
        //     "guardian_authenticators": []
        //   }
        // }

        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Users not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Patch Update a User
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Update a User"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-yellow-400 rounded-full">{"PATCH"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Update a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //               <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"
        // {
        //   "blocked": false,
        //   "email_verified": false,
        //   "email": "john.doe@gmail.com",
        //   "phone_number": "+199999999999999",
        //   "phone_verified": false,
        //   "user_metadata": {},
        //   "app_metadata": {},
        //   "given_name": "John",
        //   "family_name": "Doe",
        //   "name": "John Doe",
        //   "nickname": "Johnny",
        //   "picture": "https://secure.gravatar.com/avatar/15626c5e0c749cb912f9d1ad48dba440?s=480&r=pg&d=https%3A%2F%2Fssl.gstatic.com%2Fs2%2Fprofiles%2Fimages%2Fsilhouette80.png",
        //   "verify_email": false,
        //   "verify_phone_number": false,
        //   "password": "secret",
        //   "connection": "Initial-Connection",
        //   "client_id": "DaM8bokEXBWrTUFCiJjWn50jei6ardyX",
        //   "username": "johndoe"
        // }
        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        // "id": "id of user"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "User name updated successfully",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "User not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Delete a User
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Delete a user."}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-red-400 rounded-full">{"DELETE"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Delete a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        // "id": "id of a users"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "User deleted successfully",
        //   "data": "email of user deleted"
        // }
                  
        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Users not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Get a User Role Section
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get a user's roles"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}/roles"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"List the the roles associated with a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}/roles
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww/roles


        // "id": "id of User"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "Ok",
        //   "data": [
        //     {
        //       "id": 1,
        //       "user_id": id of user on table user,
        //       "role_id": id of role on table role,
        //       "role_identity": "role identity",
        //       "name": "name role",
        //       "description": "description role"
        //     },
        //     {
        //       "id": 2,
        //       "user_id": id of user on table user,
        //       "role_id": id of role on table role,
        //       "role_identity": "role identity",
        //       "name": "name role",
        //       "description": "description role"
        //     },
        //   ]
        // }


        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Users not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Create and assign role to a user section
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Assign roles to a user"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}/roles"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Associate roles with a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //               <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"
        // {
        //   "role_id": "id role for assign user",
        // }

        //                 "#}
        //                 </code>
        //             </pre>          
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}/roles
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww/roles

        // "id": "id of User"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 201 - Created

        // {
        //   "message": "User successfully assigned a role",
        //   "data": ""
        // }

        // Status Code 400 - Bad Request

        // {
        //   "message": "Parse error",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden

        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found

        // {
        //   "message": "Users not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error

        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
        //         </div>
        //       </div>

              // Delete a User Role
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Removes roles from a user"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-red-400 rounded-full">{"DELETE"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id_users}/role/{id_role}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Remove roles from a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id_users}/role/{id_role}
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww/role/1

        // "id_users": "id of a users"
        // "id_role": "id of a role"

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "User successfully removed from role",
        //   "data": ""
        // }

        // Status Code 400 - Bad request
        
        // {
        //   "message": "Parse error",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Users / roles not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Create and Assign Permissions to a User
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Assign Permissions to a User"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}/permissions"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Assign permissions to a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //               <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"
        // {
        //   "id's": [integer],
        // }
        //                 "#}
        //                 </code>
        //             </pre>          
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}/permissions
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww/permissions

        // "id": "id of User"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 201 - Created

        // {
        //   "message": "Successfullly assign Permissions",
        //   "data": {
        //     "id": 1,
        //     "api_id": 1,
        //     "user_id": 1,
        //     "permission_name": "name permission",
        //     "description": "description permission"
        //   }
        // }

        // Status Code 400 - Bad Request

        // {
        //   "message": "Parse error",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden

        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found

        // {
        //   "message": "Users not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error

        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
        //         </div>
        //       </div>

              // Get a User's Permissions Section
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Retrieve all permissions associated with the user."}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}/permissions"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve all permissions associated with the user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}/permissions
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww/permissions

        // "id": "id of User"

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "Succesfully get all Permissions",
        //   "data": [
        //     {
        //       "id": 1,
        //       "permission_name": "name permission",
        //       "description": "description permission",
        //       "api_name": "name api"
        //     },
        //     {
        //       "id": 2,
        //       "permission_name": "name permission",
        //       "description": "description permission",
        //       "api_name": "name api"
        //     },
        //   ]
        // }

        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Users not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Delete a User permissions 
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Remove Permissions from a User"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-red-400 rounded-full">{"DELETE"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id_users}/permissions/{id_permission}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Remove permissions from a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id_users}/permissions/{id_permission}
        // example: /api/v2/users//role/1 /api/v2/users/{AaiyAPdpYdesoKnqjj8HJqRn4T5titww}/permissions/1

        // "id_users": "id of a users"
        // "id_permission": "id of permission"

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "Successfully unassign Permission from User",
        //   "data": {
        //     "id": 1,
        //     "api_id": 1,
        //     "user_id": null,
        //     "permission_name": "name permission",
        //     "description": "description permission"
        //   }
        // }

        // Status Code 400 - Bad request
        
        // {
        //   "message": "Parse error",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Users / permission not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // get All Roles
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get roles"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/roles"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve filtered list of roles that can be assigned to users."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "Succesfully get all Roles",
        //     "data": [
        //       {
        //         "id": 1,
        //         "name": "name role",
        //         "description": "description role"
        //       },
        //       {
        //         "id": 2,
        //         "name": "name role",
        //         "description": "description role"
        //       }
        //     ]
        // }

        // Status Code 403 - Forbidden

        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error

        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       //create A Role
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Create a role"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-green-400 rounded-full">{"POST"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/roles"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Create a new role."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //               <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"
        // {
        //   "name": "name role",
        //   "description": "description role",
        // }
        //                 "#}
        //                 </code>
        //             </pre>          
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 201 - Created

        // {
        //   "message": "Successfullly create Role",
        //   "data": {
        //     "id": 1,
        //     "role_id": "role id",
        //     "name": "name role",
        //     "description": "description role",
        //     "tenant_id": 1
        //   }
        // }

        // Status Code 400 - Bad Request

        // {
        //   "message": "Parse error",
        //   "data": ""
        // }

        // Status Code 403 - Forbidden

        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error

        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }

        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       // Get Detail Roles Section
        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get a role"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-violet-400 rounded-full">{"GET"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/roles/{id}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Retrieve a role."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // response section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-lg">
        //                 {r#"



        // No Request Body Required
                        
                        

        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/roles/{id}
        // example: /api/v2/roles/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        // "id": "id of Role"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "Successfullly get Role",
        //   "data": {
        //     "id": 1,
        //     "role_id": "role id",
        //     "name": "name role",
        //     "description": "description role",
        //     "tenant_id": 1
        //   }
        // }


        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "Role not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

        //       <div class="grid grid-cols-2">
        //         // description
        //         <div class="relative items-center w-full mx-auto">
        //           <div class="grid grid-cols-1 ">
        //             <div class="w-full h-max max-w-lg mx-auto my-4 bg-white">
        //               <div class="p-4">
        //                 <h4 class="text-2xl my-2 font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Update a User"}</h4>
        //                 <span class="inline-flex items-center justify-center px-2 py-1 mr-2 text-xs font-semibold leading-none text-white bg-yellow-400 rounded-full">{"PATCH"}</span>
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"/api/v2/users/{id}"}</span>
        //                 <p class="mt-3 text-base leading-relaxed text-gray-500">
        //                 <span class="mt-3 text-base leading-relaxed text-gray-500">{"Update a user."}</span>
        //                 </p>
        //                 </div>
        //               </div>
        //             </div>
        //         </div>
                
        //         // Response Section
        //         <div class="relative items-center w-full mx-auto bg-gray-500">
        //           // Request body section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Body Example"}</h4>
        //               <h4 class="text-base font-medium leading-none text-yellow-500 tracking-tighter text-white lg:text-base ">{"Request Body Type : application/json"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"
        // {
        //   "blocked": false,
        //   "email_verified": false,
        //   "email": "john.doe@gmail.com",
        //   "phone_number": "+199999999999999",
        //   "phone_verified": false,
        //   "user_metadata": {},
        //   "app_metadata": {},
        //   "given_name": "John",
        //   "family_name": "Doe",
        //   "name": "John Doe",
        //   "nickname": "Johnny",
        //   "picture": "https://secure.gravatar.com/avatar/15626c5e0c749cb912f9d1ad48dba440?s=480&r=pg&d=https%3A%2F%2Fssl.gstatic.com%2Fs2%2Fprofiles%2Fimages%2Fsilhouette80.png",
        //   "verify_email": false,
        //   "verify_phone_number": false,
        //   "password": "secret",
        //   "connection": "Initial-Connection",
        //   "client_id": "DaM8bokEXBWrTUFCiJjWn50jei6ardyX",
        //   "username": "johndoe"
        // }
        //                 "#}
        //                 </code>
        //             </pre>      
        //           </div>
                  
        //           // Request Headers Section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Headers Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"


        // "access_token": "value from field 'token' on method POST /login"

                        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // request parameter section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Request Parameters Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // endpoint: /api/v2/users/{id}
        // example: /api/v2/users/AaiyAPdpYdesoKnqjj8HJqRn4T5titww


        // "id": "id of user"

        
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>
                  
        //           // Response section
        //           <div>
        //             <div class="p-4">
        //               <h4 class="my-2 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
        //             </div>
        //             <pre>
        //               <code class="json text-xs">
        //                 {r#"

        // Status Code 200 - Ok

        // {
        //   "message": "User name updated successfully",
        //   "data": ""

        // }

        // Status Code 403 - Forbidden
        
        // {
        //   "message": "You dont have access to this data",
        //   "data": ""
        // }

        // Status Code 404 - Not Found
        
        // {
        //   "message": "User not found",
        //   "data": ""
        // }

        // Status Code 500 - Internal Server Error
        
        // {
        //   "message": "Internal Server Error",
        //   "data": ""
        // }
        //                 "#}
        //                 </code>
        //             </pre>        
        //           </div>

        //         </div>
        //       </div>

            // Layout
            </div>
          </div>
  </>

  }
}