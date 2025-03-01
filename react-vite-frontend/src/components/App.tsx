// Make the rust router use cors to accept FE requests
// Make the rust router return a json response and display it on the FE.

/// <reference types="vite/client" />

import React from 'react';
import { useState, useEffect } from 'react';

interface ImportMetaEnv {
  readonly VITE_BACKEND_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

const { VITE_BACKEND_URL } = import.meta.env;
console.log({ VITE_BACKEND_URL });

interface ApiResponse {
  message: string;
}

const App: React.FC = () => {
  const [data, setData] = useState<ApiResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function fetchData() {
    try {
      const response = await fetch(VITE_BACKEND_URL);
      console.log(response);

      const data = await response.json();
      console.log(data);
      setData(data);
    } catch (error) {
      console.error(error);
    }
  }

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div>
      <h1>Welcome to the Vite + React + TypeScript App!</h1>
      {data && <h2>{data.message}</h2>}
    </div>
  );
};

export default App;

// import { use } from 'react';

// async function fetchData() {
//   const response = await fetch(import.meta.env.VITE_BACKEND_URL);
//   if (!response.ok) {
//     throw new Error('Failed to fetch data');
//   }
//   return response.json();
// }

// const App: React.FC = () => {
//   const data = use(fetchData());

//   return (
//     <div>
//       <h1>Welcome to the Vite + React + TypeScript App!</h1>
//       <h2>{data.message}</h2>
//     </div>
//   );
// };

// import { Suspense, use } from 'react';

// // Your async data fetching function
// async function fetchData() {
//   const response = await fetch(import.meta.env.VITE_BACKEND_URL);
//   if (!response.ok) {
//     throw new Error('Failed to fetch data');
//   }
//   return response.json();
// }

// // The component that uses the 'use' hook
// const DataComponent: React.FC = () => {
//   const data = use(fetchData());

//   return (
//     <div>
//       <h2>{data.message}</h2>
//     </div>
//   );
// };

// // The main App component wrapped with Suspense
// const App: React.FC = () => {
//   return (
//     <div>
//       <h1>Welcome to the Vite + React + TypeScript App!</h1>
//       <Suspense fallback={<div>Loading...</div>}>
//         <DataComponent />
//       </Suspense>
//     </div>
//   );
// };

// export default App;

// import { ErrorBoundary } from 'react-error-boundary';

// const App: React.FC = () => {
//   return (
//     <div>
//       <h1>Welcome to the Vite + React + TypeScript App!</h1>
//       <ErrorBoundary fallback={<div>Error loading data!</div>}>
//         <Suspense fallback={<div>Loading...</div>}>
//           <DataComponent />
//         </Suspense>
//       </ErrorBoundary>
//     </div>
//   );
// };
