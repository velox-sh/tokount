import React from 'react';

/* Greeting component */
function Greeting({ name }) {
  // Render greeting
  return (
    <div className="greeting">
      <h1>{`Hello, ${name}!`}</h1>
    </div>
  );
}
