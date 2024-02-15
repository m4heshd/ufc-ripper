<template>
  <ModMsgBox
      class="mod-support"
      vID="modSupport"
      vIcon="favorite"
      vTitle="Support this project"
  >
    <div class="mod-support__content">
      <div class="about">
        <p>
          <b>"UFC Ripper"</b> is a free, open-source tool that allows users with a paid subscription to access UFC Fight
          Pass content offline, in the format of their choice. Ideally, such services should offer these features
          without the need for third-party tools. However, since the service provider does not have these features built
          into their platform, the tool was developed to fulfill this need in a completely legal manner.
        </p>
        <p>
          It's worth mentioning that it was no easy task creating this tool. It required figuring out how UFC Fight Pass
          API backends work, countless hours of testing, and continuous maintenance for several months to reach this
          point. As the developer of this project, I do not receive any gain from it other than solely being satisfied
          by seeing people use this tool and appreciate it. Therefore, a donation would be greatly appreciated to
          support me in this endeavor.
        </p>
      </div>

      <div class="donation-sources">
        <div class="donation-sources__title">
          <b>You can send a donation my way using these methods:</b>
        </div>

        <div class="donation-sources__links center-content">
          <a
              href="https://ko-fi.variatix.net/"
              target="_blank"
              rel="noopener noreferrer"
          >
            <img
                alt="Ko-fi support button"
                title="Support me on Ko-fi"
                src="@/assets/images/ko-fi.png"
            >
          </a>
          <a
              href="https://paypal.variatix.net/"
              target="_blank"
              rel="noopener noreferrer"
          >
            <img
                alt="Paypal support button"
                title="Support me on Paypal"
                src="@/assets/images/paypal.png"
            >
          </a>
        </div>
      </div>

      <div class="donation-sources__github">
        <iframe
            src="https://github.com/sponsors/m4heshd/card"
            title="Sponsor m4heshd"
        ></iframe>
      </div>
    </div>
  </ModMsgBox>
</template>

<script setup>
// Components
import ModMsgBox from '@/components/ModMsgBox.vue';
</script>

<style lang="scss">
.mod-support {
  width: 100%;
  max-width: 600px;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0px, 1fr) max-content;
  overflow: hidden;

  &__content {
    overflow-y: auto;

    .about {
      & > p {
        text-indent: 50px;
        margin-bottom: 20px;
      }
    }

    .donation-sources {
      &__title {
        margin-bottom: 20px;
        font-size: 16rem;
        color: var(--primary);
      }

      &__links {
        margin-bottom: 20px;
        gap: 20px;

        & > a > img {
          border-radius: 0;
        }
      }

      &__github {
        border-radius: 30rem;

        & > iframe {
          width: 100%;
          height: 180px;
          border: 0;
        }
      }
    }
  }
}
</style>
