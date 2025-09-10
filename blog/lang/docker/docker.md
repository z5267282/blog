# What Drew Me To Docker

I never gave much thought about OS or hardware compatability for the first few
years of my degree.  
In coursework, the fallback was always to run code on
university lab machines which on Linux. However, after taking Front-End
programming. I realised the importance of having dependencies written as code.
In Node, a `package.json` file contains a list of dependencies that are
eventually installed to a `node_modules` folder. Importantly, this folder
should not be distributed as source code; the list of dependencies 
`package.json` should.

This concept of separating files that can be generated or installed, such as 
Python virtual environments, or compilation `build` folders in CMake-compiled
projects, is a cornerstone of distributing multi-file code projects.

However, one area where it is a little more difficult to generalise the
concept of having dependencies written is code is in binary executables. 
One of my long-term projects is a command-line video editor that needs the
binary `ffmpeg`. Anyone else wanting to run the program would need to
install this binary themselves.

What I wanted, was a way of providing a way of installing `ffmpeg` as a
dependency, akin to putting it in something like `node_modules` or `.venv`,
so that a user wouldn't have to globally install the `ffmpeg` binary, just
to run my tool.

As best summarised by [Docker's Introductory Guide](https://docs.docker.com/get-started/introduction/develop-with-containers/): "The containerized environment provide[s] the development environment, ensuring you have everything you need. You [don't] have to install ... dependencies directly on your machine. All you [need is] Docker Desktop and a code editor."

# Key Concepts

## 1. Dockerfile

The `Dockerfile` is a piece of infrastructure as code. It is somewhat analagous
to a `package.json` or `requirements.txt` in that it lays out dependencies to
install.

It is a file that contains instructions for how to install and set up all
necessary aspects of a mini computer. This includes binaries, environment 
variables, file-system structure and more.

## 2. Image

An image is created from a `Dockerfile`. It is a small blueprint containing
all the necessary binaries to run a miniature customised environment 
essentially as a stand-alone computer. Images are portable, meaning that
they can be uploaded and run on any other machine that runs the Docker client. 

Essentially it's the responsibility of the Docker application to write the 
necessary translation between code from a Docker image, to local machine 
architecture like x86 or ARM.

Each command in the Dockerfile creates a new layer in the image. 
A layer represents the total set of changes in the mini-environment created up
to a specific instruction.

## 3. Container

A container is a isolated process that runs an image as a process on a local
machine, through the Docker application.

# Commands

## Build

To build an image with a `Dockerfile` in the current directory run this command.

```sh
docker build -t app-name .
```

Here, the `-t` flag names the image `app-name` so it can be easily referred to 
when run.

## Run

```sh
docker run app-name
```

`app-name` refers to the name of an image - it can be one that was locally 
built, or from the online Docker image repository.

You can also enter the image's environment once it is up and running on the 
container, like accessing the local terminal on your device by running the 
following.

```sh
docker run -it app-name
```

The `-i` flag allows for Shell commands to be passed to the container.  
The `-t` flag creates a terminal-like environment to present access into 
the container; otherwise, there is no prompt and other expected interface features.
