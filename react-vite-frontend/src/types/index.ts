// src/types/index.ts

export interface User {
    id: number;
    name: string;
    email: string;
}

export interface Post {
    id: number;
    title: string;
    content: string;
    authorId: number;
}

export type ApiResponse<T> = {
    data: T;
    message: string;
    status: number;
};

export interface AiTool {
    id: string;
    name: string;
    company: string;
    description: string;
    image_url: string | null;
  }