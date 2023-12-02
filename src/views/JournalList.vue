<template>
    <div>
      <JournalEntry v-for="entry in entries" :key="entry.id" :entry="entry" />
    </div>
  </template>
  
  <script>
  import JournalEntry from '@/components/JournalEntry.vue';
  
  export default {
    components: {
      JournalEntry,
    },
    data() {
      return {
        entries: [],
      };
    },
    mounted() {
      // You can load entries here using methods like fetch or Tauri API
    },
  };
  </script>

<script>
import JournalEntry from '@/components/JournalEntry.vue';

export default {
  // ... (previous code)

  mounted() {
    this.loadEntries();
  },

  methods: {
    async loadEntries() {
      try {
        const entries = await this.$tauri.invoke('read_entries');
        this.entries = entries;
      } catch (error) {
        console.error("Error loading entries:", error);
      }
    },
  },
};
</script>