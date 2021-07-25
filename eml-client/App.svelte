<script>
  import Icon from "fa-svelte";
  import { faTag } from "@fortawesome/free-solid-svg-icons/faTag";
  import {
    Col,
    Container,
    Nav,
    NavItem,
    Navbar,
    NavbarBrand,
    NavLink,
    Row,
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  import EmlList from "./components/EmlList.svelte";
  import Eml from "./components/Eml.svelte";
  import Search from "./components/Search.svelte";
  import TagsSelect from "./components/TagsSelect.svelte";
  import TagQueryModal from "./components/TagQueryModal.svelte";

  let emlSelected = null;
  let tagQueries = [];
  let querySelected = "tag:inbox";

  let tagModalOpen = false;
  let tagSelected;

  const refreshTagList = () =>
    tauri.invoke("list_tags").then((tagList) => {
      tagQueries = tagList.map((t) => `tag:${t}`);
    });

  const refreshQuery = () => (querySelected = new String(querySelected));

  refreshTagList();
</script>

<Container fluid class="h-100 d-flex flex-column">
  <Row class="flex-shrink-0">
    <Navbar color="primary">
      <NavbarBrand><h1>Amail</h1></NavbarBrand>
    </Navbar>
  </Row>

  <Row class="flex-fill" style="min-height: 0;">
    <Col xs="1" class="border">
      <Nav vertical pills>
        {#each tagQueries as tag}
          <NavItem>
            <NavLink
              active={tag == querySelected}
              on:click={() => (querySelected = tag)}
            >
              <Icon icon={faTag} />
              <h2><span>{tag.split("tag:")[1]}</span></h2>
            </NavLink>
          </NavItem>
        {/each}
      </Nav>
    </Col>

    <Col xs="4" class="h-100 d-flex flex-column">
      <Row style="margin: 0.3rem">
        <Col>
          <Search bind:querySelected quietQueries={tagQueries} />
        </Col>

        <Col xs="2">
          <TagsSelect
            on:tagSelected={(tag) => {
              tagSelected = tag.detail;
              tagModalOpen = true;
            }}
          />
        </Col>

        <TagQueryModal
          bind:isOpen={tagModalOpen}
          tag={tagSelected}
          query={querySelected}
          on:retagComplete={() => {
            refreshTagList();
            refreshQuery();
          }}
        />
      </Row>

      <Row class="flex-fill mh-100 scroll">
        <EmlList
          query={querySelected}
          bind:emlSelected
          hideTags={new Set([querySelected.split("tag:")[1]])}
        />
      </Row>
    </Col>

    <Col class="bg-light mh-100 scroll">
      {#if emlSelected}
        <Eml emlMeta={emlSelected} />
      {/if}
    </Col>
  </Row>

  <Row class="flex-shrink-0">
    <Col class="mh-100" />
  </Row>
</Container>

<style scoped>
</style>
