<template>
    <RouterView />
</template>
<script setup lang="ts">
import { onMounted } from 'vue';
import { useSession } from '../presentation/composables/session';
onMounted(() => {
    void useSession()
        .load()
        .catch(() => null);
});
</script>
