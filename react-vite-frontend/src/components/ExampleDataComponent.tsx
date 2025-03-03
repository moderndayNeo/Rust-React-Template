// import { use } from 'react';

// async function fetchData() {
//   const response = await fetch(import.meta.env.VITE_BACKEND_URL);
//   if (!response.ok) {
//     throw new Error('Failed to fetch data');
//   }

//   const json = await response.json();

//   return json;
// }

// const promise = fetchData();

// export const ExampleDataComponent: React.FC = () => {
//   const data = use(promise);

//   return (
//     <div>
//       <h2>{data.message}</h2>
//     </div>
//   );
// };
