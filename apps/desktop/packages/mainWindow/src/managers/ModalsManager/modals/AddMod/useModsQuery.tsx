import {
  FEModSearchParameters,
  FEModSearchParametersQuery,
} from "@gd/core_module/bindings";
import { createStore } from "solid-js/store";

const useModsQuery = (
  initialValue?: FEModSearchParametersQuery
): [
  FEModSearchParameters,
  (_newValue: Partial<FEModSearchParametersQuery>) => void
] => {
  const [query, setQuery] = createStore<FEModSearchParameters>({
    query: initialValue || {
      categoryId: 0,
      classId: "mods",
      gameId: 432,
      gameVersion: "",
      modLoaderType: null,
      sortField: "featured",
      sortOrder: "descending",
      pageSize: 20,
      slug: "",
      searchFilter: "",
      gameVersionTypeId: null,
      authorId: null,
      index: 0,
    },
  });

  const setQueryParams = (newValue: Partial<FEModSearchParametersQuery>) => {
    const indexValue = newValue.index ?? 0;

    setQuery("query", (prev) => ({
      ...prev,
      ...newValue,
      index: indexValue,
    }));
  };

  return [query, setQueryParams];
};

export default useModsQuery;