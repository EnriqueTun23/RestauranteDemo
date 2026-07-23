import { invoke } from "@tauri-apps/api/core";

import type {
  CreateOrderInput,
  Order,
} from "./orders.types";

export async function listOrders(): Promise<Order[]> {
  return invoke<Order[]>("list_orders");
}

export async function createOrder(
  input: CreateOrderInput,
): Promise<Order> {
  return invoke<Order>("create_order", {
    input,
  });
}

export async function deleteOrder(
  orderId: string,
): Promise<void> {
  return invoke<void>("delete_order", {
    orderId,
  });
}