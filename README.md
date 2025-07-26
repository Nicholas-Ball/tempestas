# Tempestas
The Tempestas project is a Rust library with Python bindings to provide and aid in the devlopement of high preformance meterology and climate models.

## WIP
This project is a work in progress. Most of the text below is in inticpation of what the project will be.

## What is Tempestas?
Tempestas is the Roman goddess of storms, weather, and the sky. The name was chosen to reflect the project's focus on meteorology and climate modeling. It is also latin for "storm," which is fitting for a library that aims to provide high-performance tools for simulating and understanding weather patterns.

## Design Philosophy
Tempestas is designed with the following principles in mind:
- **Performance**: The library is built in Rust, a systems programming language known for its speed and safety. This allows Tempestas to handle large datasets and complex calculations efficiently. For production enviroments, I recommend using the Rust API directly with arrayfire for maximum performance which can utilize GPUs and FPGAs.
- **Compatibility**: I understand that changing programming languages can be a barrier to entry. Therefore, Tempestas provides Python bindings to make it accessible to a wider audience, including those who may not be familiar with Rust. I also work with NumPy arrays to ensure compatibility with existing Python scientific computing libraries.
- **Modularity**: The library is designed to be modular, allowing users to pick and choose the components they need for their specific use cases. This modularity also makes it easier to extend the library with new features and functionalities. I even support functions from other libraries to allow for devlopers to slowly migrate to the Rust API.

## Why isn't x feature implemented?
I am a single developer working on this project in my free time. While I strive to implement as many features as possible, there are limitations to what I can achieve. If you have a specific feature request, please open an issue on the GitHub repository, and I will do my best to prioritize it based on demand and feasibility.

I want to make sure that preformace and reliblity are maintained, especially in a high impact field like meteorology. Therefore, I may prioritize features that align with the library's core principles and have a significant impact on performance or usability.

I also am not a meteorologist, so I am not familiar with all the features that are needed for a complete meteorology library. If you have expertise in this area and would like to contribute, please feel free to reach out or submit a pull request.

## How to Contribute
Contributions are welcome! If you would like to contribute to the Tempestas project, please follow these steps:
1. Fork the repository on GitHub.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with clear, descriptive commit messages. Signed commits are not required but are nice to have.
4. Push your changes to your forked repository.
5. Open a pull request against the main repository, describing your changes and why they are needed.

## License
Tempestas is licensed under the MIT License. Which means you can use, modify, and distribute the library freely, as long as you include the original license and copyright notice in your distributions. This allows for both personal and commercial use of the library without any restrictions. See the [LICENSE](LICENSE) file for more details.

If you have conerns about the licensing or are on the fence due to the licensing, here is something that might help you. I believe strongly in helping as many people as possible no matter where they are. I hope this project is the reason someone, somewhere, has a few extra seconds or minutes to find safety. If this project is able to meet your needs in doing that or contribute to scientific devlopement, then **PLEASE** use this project and I really ask for nothing in return.
