<template>
    <iframe
        :src="`/policies/${route.params.policy}.html`"
        title="Chính sách ứng dụng O4U"
        class="policy-frame"
    />
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router';
const route = useRoute();
</script>
<style scoped>
.policy-frame {
    width: 100%;
    height: 100dvh;
    border: 0;
    display: block;
}
</style>
