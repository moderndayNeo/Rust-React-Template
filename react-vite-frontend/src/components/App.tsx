/// <reference types="vite/client" />

import React from 'react';
import { Suspense } from 'react';
import { ErrorBoundary } from 'react-error-boundary';
import { AiTools } from './AiTools';

const App: React.FC = () => {
  return (
    <div className="p-14">
      <h1 className="text-4xl text-gray-100">
        Welcome to the Vite + React + TypeScript App!
      </h1>

      <ErrorBoundary fallback={<div>Error loading data!</div>}>
        <Suspense fallback={<div>Loading...</div>}>
          <AiTools />
        </Suspense>
      </ErrorBoundary>
    </div>
  );
};

export default App;
