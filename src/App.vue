<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { VueDatePicker } from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css'
import Todo from "./models/Todo";
import TodoComp from "./components/Todo.vue";
import LanePrice from "./components/LanePrice.vue";

const todos = ref<Todo[]>([]);
const date = ref<Date>();
const text = ref("");
const showPrice = ref(false);

async function fetchTodos() {
  todos.value = await invoke("get_list");
}

async function newTodo() {
  let todo: Todo = await invoke("add_todo", { text: text.value, date: date.value?.toISOString() });
  todos.value.push(todo);
  text.value = "";
}

onMounted(() => fetchTodos());

</script>

<template>
  <main class="container">
    <a @click="() => showPrice = !showPrice">{{ showPrice ? 'Hide' : 'Show fast lane price' }}</a>
    <LanePrice v-if="showPrice" />
    <div class="header">
      <h1>Todo List: </h1>
    </div>
    <TodoComp @deleted="fetchTodos" v-for="todo in todos" :todo="todo" />
    <form class="creator" @submit.prevent="newTodo">
      <p><b>Add todo:</b></p>
      <div class="creator-inputs">
        <input v-model="text" placeholder="What to do" />
        <VueDatePicker v-model="date" :time-config="{ enableTimePicker: false }" placeholder="Deadline"></VueDatePicker>
      </div>
      <button type="submit" class="add-button">Add</button>
    </form>
  </main>
</template>

<style scoped></style>
<style>
html {
  background-color: darkblue;
  padding: 20px;
}

.header {
  width: 80%;
}

.container {
  display: flex;
  flex-direction: column;
  width: 100%;
  align-items: center;
  background-color: white;
  border-width: 1px;
  border-style: none;
  border-radius: 10px;
}

.creator {
  display: flex;
  flex-direction: column;
  width: 80%;
}

.creator p {
  margin-bottom: 0px;
}

.add-button {
  width: 30%;
  align-self: center;
  margin: 5px;
  background-color: darkblue;
  color: white;
  border-style: none;
  border-radius: 10px;
  padding-top: 10px;
  padding-bottom: 10px;
}

.creator-inputs {
  display: flex;
  flex-direction: row;
}
</style>