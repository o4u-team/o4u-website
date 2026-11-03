<template>
    <h1 class="text-h4 mb-6">Quản trị O4U</h1>
    <v-row
        ><v-col v-for="item in items" :key="item.to" cols="12" md="6"
            ><v-card :to="item.to" class="pa-6" variant="outlined"
                ><v-icon :icon="item.icon" size="32" color="primary" />
                <h2 class="text-h6 mt-3">{{ item.title }}</h2>
                <p>{{ item.description }}</p></v-card
            ></v-col
        ></v-row
    >
</template>
<script setup lang="ts">
const items = [
    {
        to: '/clients',
        title: 'Khách hàng',
        description: 'Quản lý doanh nghiệp và trạng thái hoạt động.',
        icon: 'mdi-account-group',
    },
    {
        to: '/client-systems',
        title: 'Hệ thống khách hàng',
        description: 'Endpoint Odoo, database và phân quyền ứng dụng.',
        icon: 'mdi-server',
    },
    {
        to: '/apps',
        title: 'Ứng dụng',
        description: 'Phiên bản tối thiểu, phiên bản hiện tại và liên kết tải.',
        icon: 'mdi-apps',
    },
    {
        to: '/app-usage',
        title: 'Thiết bị & lịch sử',
        description: 'Theo dõi kết nối và phiên bản trên từng thiết bị.',
        icon: 'mdi-history',
    },
];
</script>
