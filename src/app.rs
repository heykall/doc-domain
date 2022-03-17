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
        <div class="grid grid-cols-5">
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
            <div class="col-span-4 divide-y-4 divide-gray-700">
              <div class="grid grid-cols-2">
                <div class="relative items-center w-full mx-auto">
                  <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white shadow-xl rounded-xl">
                      <div class="p-6">
                        <h4 class="mt-8 text-2xl font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Introduction"}</h4>
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

              <div class="grid grid-cols-2">
                  <div class="relative items-center w-full mx-auto">
                    <div class="grid grid-cols-1 ">
                    <div class="w-full h-max max-w-lg mx-auto my-4 bg-white shadow-xl rounded-xl">
                        <div class="p-6">
                          <h4 class="mt-8 text-2xl font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl ">{"Get clients"}</h4>
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

                    
                    <div class="relative items-center w-full mx-auto bg-gray-500">
                    <h4 class="mt-8 text-lg p-5 font-medium leading-none tracking-tighter text-white lg:text-lg ">{"Response Example"}</h4>
                      <pre>
                        <code class="json text-xs">
                          {r#"
                          [
                            {
                              "client_id": "61cd1acd5498ea1bc1a60072",
                              "tenant": "dev-jefri",
                              "name": "Default App",
                              "description": "",
                              "global": false,
                              "client_secret": "string",
                              "app_type": "generic",
                              "logo_uri": "string url",
                              "is_first_party": false,
                              "oidc_conformant": false,
                              "callbacks": [
                                ""
                              ],
                              "allowed_origins": [
                                ""
                              ],
                              "web_origins": [
                                ""
                              ],
                              "client_aliases": [
                                ""
                              ],
                              "allowed_clients": [
                                ""
                              ],
                              "allowed_logout_urls": [
                                ""
                              ],
                              "grant_types": [
                                "implicit",
                                "authorization_code",
                                "refresh_token",
                                "client_credentials"
                              ],
                              "jwt_configuration": {
                                "lifetime_in_seconds": 36000,
                                "secret_encoded": false
                              },
                              "signing_keys": [
                                {
                                  "cert": "string",
                                  "pkcs7": "string",
                                  "subject": "deprecated"
                                }
                              ],
                              "sso": false,
                              "sso_disabled": false,
                              "cross_origin_auth": false,
                              "cross_origin_loc": "",
                              "custom_login_page_on": false,
                              "custom_login_page": "",
                              "custom_login_page_preview": "",
                              "form_template": "",
                              "token_endpoint_auth_method": "none",
                              "initiate_login_uri": "",
                              "is_token_endpoint_ip_header_trusted": false,
                              "refresh_token": {
                                "expiration_type": "non-expiring",
                                "leeway": 0,
                                "infinite_token_lifetime": false,
                                "infinite_idle_token_lifetime": false,
                                "token_lifetime": 2592000,
                                "idle_token_lifetime": 1296000,
                                "rotation_type": "non-rotating"
                              },
                              "organization_usage": "deny",
                              "organization_require_behavior": "no_prompt",
                              "encrypted": true,
                              "callback_url_template": false
                            }
                          ]                          
                          "#}
                          </code>
                      </pre>        
                    </div>
              </div>


            </div>
          </div>
            // <div class="col-span-2">{"03"}</div>
    </>
 
    }
}

// pub struct Basic {}

// pub enum Msg {}

// impl Component for Basic {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Basic {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         true
//     }

//     fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
//         if first_render {
//             // ConsoleService::info("This is first render in app tab settings");
//             // self.link.send_message(Msg::RequestAppList);
//             highlight();
//         }
//     }

//     fn changed(&mut self, _ctx: &Context<Self>) -> bool {
//         false
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <>
//             <pre><code class="json">{r#"{
//                           "FirstName": "John",
//                           "LastName": "Doe",
//                           "Age": 43,
//                           "Address": {
//                               "Street": "Downing Street 10",
//                               "City": "London",
//                               "Country": "Great Britain"
//                           },
//                           "PhoneNumbers": [
//                               "+44 1234567",
//                               "+44 2345678"
//                           ]
//                       }"#
//                        }</code></pre>
                
//                        <pre><code class="arcade">{"Hello World"
//                      }</code></pre>
//             </>
//         }
//     }
// }