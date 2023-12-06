<template>
  <form @submit.prevent="addEntry">
    <textarea v-model="entryText" id="entryText" rows="4"></textarea>
    <br>
    <button type="submit">Add Entry</button>
  </form>
</template>

<script>
export default {
  data() {
    return {
      entryText: '',
    };
  },
  methods: {
    addEntry() {
      // Call a function to send the entryText to the Tauri back-end
      this.$tauri
        .invoke('new_entry', { text: this.entryText })
        .then(response => {
          // Handle the response if needed
          console.log(response);
          // Assuming the response contains the updated list of entries
          this.$emit('entry-added', response.entries);
        })
        .catch(error => {
          console.error(error);
          // Handle errors if needed
        });
      // Clear the entryText after submitting
      this.entryText = '';
    },
  },
};
</script>

<style scoped>
/* Add component-specific styles here */
</style>
