// App.vue

<template>
  <div id="app">
    <Header />
    <br />
    <NewEntryForm @entry-added="updateEntryList" />
    <EntryList :entries="entries" />
  </div>
</template>

<script>
import Header from '@/components/Header.vue';
import NewEntryForm from '@/components/NewEntryForm.vue';
import EntryList from '@/components/EntryList.vue';

export default {
  components: {
    Header,
    NewEntryForm,
    EntryList,
  },
  data() {
    return {
      entries: [], // Initialize with an empty array
    };
  },
  methods: {
    updateEntryList(newEntries) {
  console.log('Updating entry list with:', newEntries);

  // Update the entry list with the new entries
  this.entries = newEntries;

  // Assuming you want to persist entries to a JSON file after each update
  this.saveEntriesToFile();

  console.log('Entry list updated successfully');
},
    saveEntriesToFile() {
      // Call Tauri function to save entries to a JSON file
      this.$tauri
        .invoke('save_entries_to_file', { entries: this.entries })
        .then(response => {
          // Handle the response if needed
          console.log(response);
        })
        .catch(error => {
          console.error(error);
          // Handle errors if needed
        });
    },
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
