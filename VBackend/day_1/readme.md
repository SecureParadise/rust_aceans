# Simple Express.js Application

This project is a basic Express.js application demonstrating fundamental routing and environment variable usage.

## Setup

1.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    ```
2.  **Navigate to the project directory:**
    ```bash
    cd <project_directory>
    ```
3.  **Install dependencies:**
    ```bash
    npm install
    ```
4.  **Create a `.env` file:**
    -   Create a file named `.env` in the root directory.
    -   Add the `PORT` variable (e.g., `PORT=4000`). If not provided the app will default to port 4000.
5.  **Start the server:**
    ```bash
    npm start
    ```
    (You may need to add `"start": "node index.js"` in the scripts section of your package.json file)

## Dependencies

-   **Express.js:** A fast, minimalist web framework for Node.js.
-   **dotenv:** Loads environment variables from a `.env` file into `process.env`.

## Project Structure

-   `index.js`: The main application file containing the Express.js server and routes.
-   `.env`: (Optional) Stores environment variables (e.g., `PORT`).
-   `node_modules`: Directory containing installed npm packages.
-   `package.json`: Project metadata and dependencies.
-   `package-lock.json`: Records the exact versions of installed packages.
-   `.gitignore`: Specifies files and directories to be ignored by Git (e.g., `node_modules`, `.env`).
-   `README.md`: Project documentation.

## Routes

-   `/`: Returns "Hello World!".
-   `/login`: Returns an HTML heading "Login Page".
-   `/yt`: Returns an HTML button.
-   `/x`: Returns "Hello Musky".

## Key Concepts

-   **Express.js:** Used for creating web servers and handling HTTP requests.
-   **dotenv:** Used for managing environment variables, keeping sensitive information out of the code.
-   **Routing:** Defining how the server responds to different URL paths.
-   **.gitignore:** Preventing sensitive or unnecessary files from being committed to version control.

## Explanation of the process

1.  **Initialize Node.js project:**
    -   `npm init` was used to create a `package.json` file.
2.  **Install Express.js:**
    -   `npm install express` was used to install the Express.js framework.
3.  **Install dotenv:**
    -   `npm install dotenv` was used to install the dotenv package.
4.  **Create `.gitignore`:**
    -   A `.gitignore` file was created to exclude `node_modules` and `.env` from version control.
5.  **Implement basic Express.js routes:**
    -   Simple routes were defined to respond with different messages and HTML elements.
6.  **Use dotenv to load environment variables:**
    -   `require('dotenv').config()` was used to load variables from the `.env` file.
7.  **Run the server:**
    -   The server was started using `node index.js`. Or `npm run start` if that script is added to package.json.