<template>
  <div>
    <Person />
    <Person />
  </div>
</template>


<script lang="ts">
import Person from './components/Person.vue'
export default {
  name: 'app',
  components: { Person }
}
</script>

<style>
.app {
  background-color: blue;
  box-shadow: 0 0 10px;
  border-radius: 10px;
  padding: 20px
}
</style>