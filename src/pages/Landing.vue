<template>
  <div class="ufcr">
    <div class="logo center-content">
      <img
          alt="UFC Ripper logo"
          src="@/assets/images/ufcr-logo.svg"
      >
    </div>

    <div class="url-section center-content">
      <div class="field label border round url-section__txt-link">
        <input
            type="text"
            autocomplete="off"
        >
        <label>Link</label>
        <span class="helper">Paste the direct link to Fight Pass video here</span>
      </div>
      <button class="square round large">
        <i>download</i>
      </button>
      <button class="square round large">
        <i>settings</i>
      </button>
    </div>
  </div>
</template>

<style lang="scss">
@use "~@/assets/styles/app.scss";
@use "~@/assets/styles/common.scss";
@use "~@/assets/styles/overrides.scss";

.ufcr {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 50px;
  margin: 4vw;

  .logo > img {
    height: 80px;
  }

  .url-section {
    gap: 10px;

    &__txt-link {
      width: 60vw;
      max-width: 900px;
      margin-bottom: 0;
    }
  }
}
</style>
