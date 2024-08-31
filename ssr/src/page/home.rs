use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
                <main>



                <section class="relative text-white bg-center bg-cover h-[500px]" style="background-image: url('hero-bg.jpg');">
      <div class="absolute inset-0 bg-black opacity-50"></div>

      <div class="flex absolute inset-x-0 top-0 z-20 justify-center items-center">
        <div class="relative w-full bg-white max-w-[756.75px] rounded-b-[75px]">
          <ul class="flex justify-center items-center gap-[10%] list-none p-2 uppercase text-black">
            <li><a href="#">Home</a></li>
            <li><a href="#">Products</a></li>
            <li><a href="#">Contact</a></li>
          </ul>
        </div>
        <div class="absolute top-0 z-50 bg-yellow-500 transform left-[-30px] h-[10px] w-[10px] rotate-[-20deg] rounded-tr-[20px]"></div>
      </div>

      <div class="flex relative z-10 items-center mx-auto max-w-screen-xl h-full">
        <div class="p-8">
          <h1 class="text-5xl font-bold">Explore your place to stay</h1>
          <form class="flex mt-8 space-x-4">
            <input type="text" placeholder="Add your location" class="p-4 w-full text-gray-900" />
            <input type="date" class="p-4 text-gray-900" />
            <input type="time" class="p-4 text-gray-900" />
            <input type="date" class="p-4 text-gray-900" />
            <input type="time" class="p-4 text-gray-900" />
            <button class="py-4 px-6 text-white bg-green-600 hover:bg-green-700">Search</button>
          </form>
        </div>
      </div>
    </section>

                    <section class="py-16 bg-gray-100">
                        <div class="mx-auto max-w-screen-xl text-center">
                            <h2 class="mb-8 text-3xl font-bold">Top spots near Bangalore</h2>
                            <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
                                <div class="overflow-hidden bg-white rounded-lg shadow-lg">
                                    <img src="spot1.jpg" alt="Spot 1" class="object-cover w-full h-48" />
                                    <div class="p-4">
                                        <h3 class="text-xl font-bold">Chikmagalur</h3>
                                        <p class="text-gray-600">$199</p>
                                    </div>
                                </div>
                                <div class="overflow-hidden bg-white rounded-lg shadow-lg">
                                    <img src="spot2.jpg" alt="Spot 2" class="object-cover w-full h-48" />
                                    <div class="p-4">
                                        <h3 class="text-xl font-bold">Mangalore</h3>
                                        <p class="text-gray-600">$300</p>
                                    </div>
                                </div>
                                <div class="overflow-hidden bg-white rounded-lg shadow-lg">
                                    <img src="spot3.jpg" alt="Spot 3" class="object-cover w-full h-48" />
                                    <div class="p-4">
                                        <h3 class="text-xl font-bold">Chennai</h3>
                                        <p class="text-gray-600">$250</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="py-16 bg-white">
                        <div class="mx-auto max-w-screen-xl text-center">
                            <h2 class="mb-8 text-3xl font-bold text-green-600">ADVANTAGES</h2>
                            <p class="mb-12 text-gray-600">Why Choose Us?</p>
                            <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
                                <div class="flex items-center space-x-4">
                                    <div class="p-4 bg-green-100 rounded-full">
                                        <svg class="w-8 h-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c1.104 0 2-.896 2-2s-.896-2-2-2-2 .896-2 2 .896 2 2 2zM4 12h8M12 4v8" />
                                        </svg>
                                    </div>
                                    <div>
                                        <h4 class="text-xl font-bold">Easy Rent</h4>
                                        <p class="text-gray-600">Rent a car with ease.</p>
                                    </div>
                                </div>
                                <div class="flex items-center space-x-4">
                                    <div class="p-4 bg-green-100 rounded-full">
                                        <svg class="w-8 h-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9v3m4 0v-3m-4 0h4M5 3h14a2 2 0 012 2v14a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2z" />
                                        </svg>
                                    </div>
                                    <div>
                                        <h4 class="text-xl font-bold">Premium Quality</h4>
                                        <p class="text-gray-600">Our cars are always maintained.</p>
                                    </div>
                                </div>
                                <div class="flex items-center space-x-4">
                                    <div class="p-4 bg-green-100 rounded-full">
                                        <svg class="w-8 h-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c1.104 0 2-.896 2-2s-.896-2-2-2-2 .896-2 2 .896 2 2 2zM4 12h8M12 4v8" />
                                        </svg>
                                    </div>
                                    <div>
                                        <h4 class="text-xl font-bold">Professional Agent</h4>
                                        <p class="text-gray-600">Expert agents to assist you.</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="py-16 bg-green-100">
                        <div class="mx-auto max-w-screen-xl text-center">
                            <h2 class="mb-8 text-3xl font-bold">Invest in a Car Network</h2>
                            <p class="mb-8 text-gray-600">If you need consultation with us, you can write a message or call us. We will respond as quickly as possible.</p>
                            <button class="py-4 px-6 text-white bg-green-600 hover:bg-green-700">Coming Soon</button>
                        </div>
                    </section>
                </main>
            }
}
