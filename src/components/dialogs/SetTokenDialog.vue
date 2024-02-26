<script setup>
import { ref } from "vue";

const dialog = defineModel();
const emit = defineEmits(["submit"]);

const token = ref("");
</script>

<template>
  <v-dialog v-model="dialog">
    <v-card>
      <v-card-title title="Set CMC Token" />
      <v-card-text>
        Type the CoinMarketCap API token.
        <v-text-field v-model="token" label="Token" />
      </v-card-text>
      <v-card-actions>
        <v-btn @click.stop="dialog = !dialog" text="Cancel" />
        <v-spacer />
        <v-btn @click.stop="emit('submit', token)" text="Set" />
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
