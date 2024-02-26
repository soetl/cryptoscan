<script setup>
import { ref } from "vue";

const dialog = defineModel();
const emit = defineEmits(["submit"]);

const symbol = ref("");

function addCoin() {
  emit("submit", symbol.value);
  dialog.value = false;
  symbol.value = "";
}
</script>

<template>
  <v-dialog v-model="dialog">
    <v-card>
      <v-card-title title="Add Coin" />
      <v-card-text>
        Type the CoinMarketCap symbol of the coin you want to add.
        <v-text-field v-model="symbol" label="Symbol" />
      </v-card-text>
      <v-card-actions>
        <v-btn @click.stop="dialog = !dialog" text="Cancel" />
        <v-spacer />
        <v-btn @click.stop="addCoin" text="Add" />
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
