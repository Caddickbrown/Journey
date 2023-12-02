<template>
    <div>
      <h2>{{ entry.title }}</h2>
      <p>{{ entry.content }}</p>
      <!-- Add more fields as needed -->
    </div>
  </template>
  
  <script>
  export default {
    props: ['entry'],
  };
  </script>