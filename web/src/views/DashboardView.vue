  <template>
    <div style="display: flex; flex-direction:column;">
      <PaletteRow color="amber" count="9" />
      <PaletteRow color="brown" count="9" />

    </div>

  </template>

<script setup lang="ts">
  import PaletteRow from '../components/PaletteRow.vue';
  //import { getBooks } from "../services/books"

  // const books = await getBooks({
  //   page: 1,
  //   page_size: 20,
  // });

</script>

<style lang="css" scoped>

  .book-grid-holder {
    /* display: grid; */
    display: flex;
    margin: 2rem;
    width: 100%;
    /* grid-template-columns: repeat(auto-fit); */
  }

</style>
