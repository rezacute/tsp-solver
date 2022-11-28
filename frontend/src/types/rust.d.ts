/* This file is generated and managed by tsync */

interface Todo {
  id: number
  text: string
  created_at: Date
}

interface TodoForm {
  text: string
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface TspSet {
  id: number
  label: string
  created_at: Date
}

interface TspSetForm {
  label: string
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface PaginationParams {
  page: number
  page_size: number
}

interface GeoPoint {
  lat: number
  lng: number
  label: string
}
