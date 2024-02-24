<script setup>
import { mdiPencil, mdiPlus } from '@mdi/js';
</script>

<template>
  <v-app>
    <v-layout>
      <v-app-bar>
        <template v-slot:prepend>
          <v-app-bar-nav-icon />
        </template>

        <v-app-bar-title text="cryptoscan" />

        <template v-slot:append>
          <v-btn :icon="mdiPencil" />
          <v-btn :icon="mdiPlus" />
        </template>
      </v-app-bar>

      <v-navigation-drawer>
        <v-list>
          <v-list-item title="Force reload" />
          <v-list-item title="Change API key" />
          <v-list-item title="Clear all coins" />
          <v-list-item title="Toggle theme" />
        </v-list>
      </v-navigation-drawer>

      <v-main>
        
      </v-main>
    </v-layout>
  </v-app>
</template>