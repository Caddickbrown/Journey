<template>
  <section>
    <h2>Journal Entries</h2>
    <ul>
      <li v-for="entry in entries" :key="entry.id">{{ entry.text }}</li>
    </ul>
  </section>
</template>

<script>
export default {
  data() {
    return {
      entries: [
        // You'll replace this with actual data retrieved from Tauri
        { id: 1, text: 'Sample Entry 1' },
        { id: 2, text: 'Sample Entry 2' },
      ],
    };
  },
  // Add component-specific logic here if needed
};
</script>

<style scoped>
section {
  margin-top: 20px;
}

h2 {
  color: #333333;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  margin: 10px 0;
  padding: 8px;
  border: 1px solid #cccccc;
  border-radius: 4px;
}
</style>
