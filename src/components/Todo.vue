<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import Todo from '../models/Todo';
import { computed } from 'vue';

const emit = defineEmits(['deleted']);
const props = defineProps<{
    todo: Todo
}>();

async function finishTodo() {
    await invoke("finish_todo", {deleteId: props.todo.id});
    emit('deleted');
}

const isDue = computed(() => new Date() > new Date(props.todo.date));
</script>

<template>
    <div class="todo" :class="{ due: isDue}">
        <div class="data">
            <p>{{ props.todo.text }}</p>
        </div>
        <div class="data"><p>Do by <b>{{ props.todo.date }}</b></p></div>
        <button class="done-button" @click="finishTodo">Done</button>
    </div>
</template>

<style scoped>

</style>

<style>
.done-button {
    background-color: green;
    color: white;
    font-weight: bold;
    border-style: none;
    border-radius: 10px;
}

.data {
    width: 30%;
}

.due {
    text-shadow: 0 0 2px red;
}

.todo {
    display: flex;
    flex-direction: row;
    justify-content: center;
    margin: 2px;
    width: 100%;
}
</style>