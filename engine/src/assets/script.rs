use bevy::reflect::{TypePath, TypeUuid};

use crate::prelude::*;
use crate::time::TimeSpec;

/// Scripted Sequence Asset type
///
/// Would typically be loaded from TOML files.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(TypeUuid, TypePath)]
#[uuid = "8D1B7F2F-3798-4438-9EB8-A5EEC3EA77A9"]
pub struct Script {
    /// List of actions to perform during playback
    pub script: Vec<CommonScript>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CommonScript {
    #[serde(flatten)]
    pub run_if: CommonScriptRunIf,
    #[serde(flatten)]
    pub action: CommonScriptAction,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum CommonScriptRunIf {
    #[serde(rename = "run_at_tick")]
    Tick(u64),
    #[serde(rename = "run_every_n_ticks")]
    TickQuant(TickQuant),
    #[serde(rename = "run_at_time")]
    Time(TimeSpec),
    #[serde(rename = "run_at_millis")]
    Millis(u64),
}

/// The various actions that can be performed from scripts
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum CommonScriptAction {
    /// Run `CliCommand`s
    RunCli {
        /// A list of cli command strings to evaluate
        cli: Vec<String>,
    },
    /// Despawn entities
    DespawnEntity {
        /// If specified, lookup entities with the given label.
        /// If unspecified, despawn ourselves.
        label: Option<String>,
    },
    /// Spawn a Bevy Scene asset
    SpawnScene {
        /// The dynamic asset key of the scene asset to spawn
        scene_asset_key: String,
        /// If true, spawn it as a child under another entity.
        /// If false, spawn it independently (standalone).
        #[serde(default)]
        as_child: bool,
        /// If `as_child` is true, optionally specify another entity (by label)
        /// to use as the parent. If unspecified, use the current entity.
        parent_label: Option<String>,
    },
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(from = "ExtendedScriptWorkaround<ExtRunIf, ExtAction>")]
pub struct ExtendedScript<ExtRunIf, ExtAction> {
    #[serde(flatten)]
    pub run_if: ExtendedScriptRunIf<ExtRunIf>,
    #[serde(flatten)]
    pub action: ExtendedScriptAction<ExtAction>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtendedScriptRunIf<ExtRunIf> {
    Extended(ExtRunIf),
    Common(CommonScriptRunIf),
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtendedScriptAction<ExtAction> {
    Extended(ExtAction),
    Common(CommonScriptAction),
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Flattened<A, B> {
    #[serde(flatten)]
    a: A,
    #[serde(flatten)]
    b: B,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtendedScriptWorkaround<ExtRunIf, ExtAction> {
    EE(Flattened<ExtRunIf, ExtAction>),
    EC(Flattened<ExtRunIf, CommonScriptAction>),
    CE(Flattened<CommonScriptRunIf, ExtAction>),
    CC(Flattened<CommonScriptRunIf, CommonScriptAction>),
}

impl<ExtRunIf, ExtAction> From<ExtendedScriptWorkaround<ExtRunIf, ExtAction>>
    for ExtendedScript<ExtRunIf, ExtAction>
{
    fn from(
        wa: ExtendedScriptWorkaround<ExtRunIf, ExtAction>,
    ) -> ExtendedScript<ExtRunIf, ExtAction> {
        match wa {
            ExtendedScriptWorkaround::EE(x) => {
                ExtendedScript {
                    run_if: ExtendedScriptRunIf::Extended(x.a),
                    action: ExtendedScriptAction::Extended(x.b),
                }
            },
            ExtendedScriptWorkaround::EC(x) => {
                ExtendedScript {
                    run_if: ExtendedScriptRunIf::Extended(x.a),
                    action: ExtendedScriptAction::Common(x.b),
                }
            },
            ExtendedScriptWorkaround::CE(x) => {
                ExtendedScript {
                    run_if: ExtendedScriptRunIf::Common(x.a),
                    action: ExtendedScriptAction::Extended(x.b),
                }
            },
            ExtendedScriptWorkaround::CC(x) => {
                ExtendedScript {
                    run_if: ExtendedScriptRunIf::Common(x.a),
                    action: ExtendedScriptAction::Common(x.b),
                }
            },
        }
    }
}