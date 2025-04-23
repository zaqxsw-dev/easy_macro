<a name="readme-top"></a>



<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">easy_macro</h3>

  <p align="center">
    Most usefull rust macros
    <br />
    <a href="https://github.com/zaqxsw-dev/easy_macro"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/zaqxsw-dev/easy_macro/issues">Report Bug</a>
    ·
    <a href="https://github.com/zaqxsw-dev/easy_macro/issues">Request Feature</a>
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

I have created a library for combine all rust macro what i use in one place.
I tryed make this macro max readable.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

Just add new dependencie in your Cargo.toml file.

* Cargo.toml
  ```toml
  [dependencies]
  easy_macro = "0.1"
  ```

<!-- USAGE EXAMPLES -->
## Usage

### Builder pattern implementation

```rust
use easy_macro::EasyBuilder;

#[derive(EasyBuilder)]
struct MyStruct {
    field1: i32,
    field2: String,
}

impl Default for MyStruct {
  fn default() -> Self {
    Self {
      field1: 0,
      field2: String::from("123123")
    }
  }
}

fn main() {
  let builded_struct = MyStruct::default().set_field1(42).set_field2(String::from("pewpew"));
}
```

_For more examples, please refer to the [Documentation](https://docs.rs/easy_macro/0.1.0/easy_macro/)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Bogdan Lipovtsev - megafreelancer2012@gmail.com

Project Link: [https://github.com/zaqxsw-dev/easy_macro](https://github.com/zaqxsw-dev/easy_macro)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/zaqxsw-dev/easy_macro.svg?style=for-the-badge
[contributors-url]: https://github.com/zaqxsw-dev/easy_macro/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/zaqxsw-dev/easy_macro.svg?style=for-the-badge
[forks-url]: https://github.com/zaqxsw-dev/easy_macro/network/members
[stars-shield]: https://img.shields.io/github/stars/zaqxsw-dev/easy_macro.svg?style=for-the-badge
[stars-url]: https://github.com/zaqxsw-dev/easy_macro/stargazers
[issues-shield]: https://img.shields.io/github/issues/zaqxsw-dev/easy_macro.svg?style=for-the-badge
[issues-url]: https://github.com/zaqxsw-dev/easy_macro/issues
[license-shield]: https://img.shields.io/github/license/zaqxsw-dev/easy_macro.svg?style=for-the-badge
[license-url]: https://github.com/zaqxsw-dev/easy_macro/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/bogdan-lipovtsev-746946257
