# Jokes App

## Project Overview
This project consists of two main components:
1. **Backend**: Serves JSON data containing jokes.
2. **Frontend**: Fetches and displays jokes using React.

## Folder Structure
```
/jokes-app
  ├── backend  # Node.js server serving jokes data
  ├── frontend # React app rendering jokes
```

## Backend
- The backend serves a JSON array of jokes via an API endpoint.
- There are two types of jokes:
  - **Single-part jokes**: Direct jokes.
  - **Two-part jokes**: Contain a `setup` and a `delivery`.
- The JSON data is available at:
  ```
  GET /api/jokes
  ```
- Technologies Used:
  - Node.js
  - Express.js

## Frontend
- The frontend is built using React.
- It fetches jokes from the backend and displays them dynamically.
- We used the following commands to set up the frontend:
  ```sh
  npm create vite@latest
  ```
- We configured a proxy in `vite.config.js` to avoid CORS issues:
  ```js
  server: {
    proxy: {
      '/api': 'http://localhost:5000'
    }
  }
  ```
- We used **useState** and **useEffect** to manage and render jokes dynamically.
- Technologies Used:
  - React.js
  - JavaScript

## Key Learning
- Initially, I was able to fetch backend data from the frontend **even when the backend server was not actively running**.
- Upon investigation, I found that the backend process was still running in the background.
- Running the following command showed active processes:
  ```sh
  tasklist | findstr node  # Windows
  ps aux | grep node       # Linux/macOS
  ```
- After killing the process manually using:
  ```sh
  taskkill /IM node.exe /F  # Windows
  killall node              # Linux/macOS
  ```
  the frontend could no longer fetch data until the backend was restarted.

## How to Run the Project
1. Start the backend:
   ```sh
   cd backend
   npm install
   node server.js
   ```
2. Start the frontend:
   ```sh
   cd frontend
   npm install
   npm run dev
   ```
3. Open `http://localhost:5173` (or the port Vite provides) in your browser.

## Conclusion
To fetch jokes successfully, **both the frontend and backend must be running**. If the backend seems to be fetching data without running, check for any lingering processes using `tasklist` or `ps aux` and terminate them before restarting the server.

