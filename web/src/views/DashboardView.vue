  <template>

    <div style="display: grid">
      <div v-for="book in books.data">
        <span style="font-family:DM Sans; color:var(--amber-200);">{{book.book.sort_title}}</span>
      </div>
    </div>


</template>

<script setup lang="ts">
import {getBooks} from "../services/books"

const books = await getBooks({
  page:1,
  page_size:20,
});

console.log(books.data);


</script>
