<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

import { mdiPencil, mdiPlus } from '@mdi/js';

import CoinCard from './components/CoinCard.vue';

const theme = ref('light');
const navDrawer = ref(false);
const editMode = ref(false);

const coins = ref([]);
coins.value.push({
  id: 1,
  name: "Bitcoin",
  symbol: "BTC",
  price: 0,
  percent_change_24h: 0,
});

function removeCoin(id, symbol) {
  coins.value = coins.value.filter((coin) => coin.id !== id);
}

function invokeGet() {
  invoke("get_coins").then((result) => {
    coins.value = result;
  });
}
</script>

<template>
  <v-app :theme="theme">
    <v-layout>
      <v-app-bar>
        <template v-slot:prepend>
          <v-app-bar-nav-icon @click.stop="navDrawer = !navDrawer" />
        </template>

        <v-app-bar-title text="cryptoscan" />

        <template v-slot:append>
          <v-btn :icon="mdiPencil" @click.stop="editMode = !editMode" />
          <v-btn :icon="mdiPlus" @click.stop="invokeGet" />
        </template>
      </v-app-bar>

      <v-navigation-drawer v-model="navDrawer">
        <v-list>
          <v-list-item title="Force update" />
          <v-list-item title="Change API key" />
          <v-list-item title="Clear all coins" />
          <v-list-item title="Toggle theme" @click.stop="theme = theme === 'light' ? 'dark' : 'light'" />
        </v-list>
      </v-navigation-drawer>

      <v-main>
        <coin-card v-for="coin in coins" :key="coin.id" :coin="coin" :editMode="editMode" @removeCoin="removeCoin" />
      </v-main>
    </v-layout>
  </v-app>
</template>
