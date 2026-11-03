<template>
    <v-dialog
        :model-value="modelValue"
        max-width="360"
        scrim="rgba(6, 24, 44, 0.55)"
        @update:model-value="emit('update:modelValue', $event)"
    >
        <v-card class="zalo-dialog" rounded="xl" elevation="0">
            <v-btn
                class="zalo-dialog__close"
                icon
                variant="flat"
                size="small"
                color="white"
                aria-label="Đóng"
                @click="emit('update:modelValue', false)"
            >
                <v-icon>mdi-close</v-icon>
            </v-btn>

            <img
                class="zalo-dialog__qr"
                :src="qrUrl"
                :alt="`Mã QR Zalo của ${displayName}`"
                loading="lazy"
            />
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
defineProps<{
    modelValue: boolean;
    qrUrl: string;
    displayName: string;
}>();

const emit = defineEmits<{
    'update:modelValue': [value: boolean];
}>();
</script>

<style scoped>
.zalo-dialog {
    position: relative;
    overflow: hidden;
    background: #0068ff;
    line-height: 0;
}

.zalo-dialog__close {
    position: absolute;
    top: 0.65rem;
    right: 0.65rem;
    z-index: 1;
    background: rgba(0, 0, 0, 0.28) !important;
    color: #fff !important;
}

.zalo-dialog__qr {
    display: block;
    width: 100%;
    height: auto;
    margin: 0 auto;
    object-fit: contain;
    object-position: center;
}
</style>
