<a name="readme-top"></a>

[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Unlicense][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">Convert Temp</h3>

  <p align="center">
    Convert temperatures between Fahrenheit, Celsius, and Kelvin.
    <br />
    <a href="https://github.com/jasoncavinder/convert_temp"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jasoncavinder/convert_temp/issues">Report Bug</a>
    ·
    <a href="https://github.com/jasoncavinder/convert_temp/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

![Convert Temp Screen Shot][product-screenshot]

I'm learning Rust by working through ["The Book"](https://doc.rust-lang.org/stable/book/) and building simple projects to practice the concepts discussed in various chapters. This is one of those projects.

### This project was based on the following prompt:

You made it! This was a sizable chapter: you learned about variables, scalar and compound data types, functions, comments, if expressions, and loops! To practice with the concepts discussed in this chapter, try building programs to do the following:

- [x] Convert temperatures between Fahrenheit and Celsius.
- [ ] Generate the nth Fibonacci number.
- [ ] Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

- [![Rust][rust-lang]][rust-url]
- [![VS Code][vs-code]][vs-code-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

Rust

- Linux/macOS
  ```sh
  curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
  ```
- Windows  
  [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

\[Optional\] Install a C compiler

- Linux
  ```sh
  sudo apt install build-essential
  ```
- macOS
  ```sh
  xcode-select --install
  ```
- Windows  
  [https://visualstudio.microsoft.com/downloads/](https://visualstudio.microsoft.com/downloads/)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/jasoncavinder/convert_temp.git
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Build & Run

```sh
cargo run
```

```
Convert temperatures between Fahrenheit, Celsius, and Kelvin.
Enter a temperature and a unit (F, C, or K) to convert to the other units.
98.6F
98.6 °Fahrenheit is 37.0 °Celsius and 310.15 Kelvin.

Enter another temperature to convert or press enter to exit.
Enter a temperature and a unit (F, C, or K) to convert to the other units.
55C
55.0 °Celsius is 131.0 °Fahrenheit and 328.15 Kelvin.

Enter another temperature to convert or press enter to exit.
Enter a temperature and a unit (F, C, or K) to convert to the other units.
100K
100.0 Kelvin is -173.15 °Celsius and -279.67 °Fahrenheit.

Enter another temperature to convert or press enter to exit.
Enter a temperature and a unit (F, C, or K) to convert to the other units.
-500F
Temperature is below absolute zero.

Enter a temperature and a unit (F, C, or K) to convert to the other units.
0k
0.0 Kelvin is -273.15 °Celsius and -459.67 °Fahrenheit.

Enter another temperature to convert or press enter to exit.
Enter a temperature and a unit (F, C, or K) to convert to the other units.
-10C
-10.0 °Celsius is 14.0 °Fahrenheit and 263.15 Kelvin.

Enter another temperature to convert or press enter to exit.
Enter a temperature and a unit (F, C, or K) to convert to the other units.

Exiting.
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

- [ ] Add comments and documentation
- [ ] Optimize code

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the Unlicense. See `UNLICENSE.md` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGEMENTS -->

## Acknowledgements

(book) [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Jason Cavinder - [@jasoncavinder](https://twitter.com/jasoncavinder) - jason.cavinder@gmail.com - [Jason Cavinder][linkedin-url]

Project Link: [https://github.com/jasoncavinder/convert_temp](https://github.com/jasoncavinder/convert_temp)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/jasoncavinder/convert_temp.svg?style=for-the-badge
[contributors-url]: https://github.com/jasoncavinder/convert_temp/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jasoncavinder/convert_temp.svg?style=for-the-badge
[forks-url]: https://github.com/jasoncavinder/convert_temp/network/members
[stars-shield]: https://img.shields.io/github/stars/jasoncavinder/convert_temp.svg?style=for-the-badge
[stars-url]: https://github.com/jasoncavinder/convert_temp/stargazers
[issues-shield]: https://img.shields.io/github/issues/jasoncavinder/convert_temp.svg?style=for-the-badge
[issues-url]: https://github.com/jasoncavinder/convert_temp/issues
[license-shield]: https://img.shields.io/github/license/jasoncavinder/convert_temp.svg?style=for-the-badge
[license-url]: https://github.com/jasoncavinder/convert_temp/blob/master/UNLICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/jason-cavinder
[product-screenshot]: images/screenshot.png
[rust-lang]: https://img.shields.io/badge/Rust-120712?style=for-the-badge&logo=Rust&logoColor=B94700
[rust-url]: https://www.rust-lang.org/
[vs-code]: https://img.shields.io/badge/VS_Code-000?style=for-the-badge&logo=visualstudiocode&logoColor=0078D7
[vs-code-url]: https://code.visualstudio.com/

<!--
README.md based on [othneildrew/Best-README-Template]: https://github.com/othneildrew/Best-README-Template

MIT License

Copyright (c) 2021 Othneil Drew

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. -->
