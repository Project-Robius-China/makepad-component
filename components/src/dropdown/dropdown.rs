use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::radius::*;

    // Default style dropdown with light theme popup
    pub MpDropdown = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 1.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #FFFFFF
            color_hover: #FAFAFA
            color_focus: #FFFFFF
            color_down: #F5F5F5
            color_disabled: #F5F5F5

            border_color: #e5e5e5
            border_color_hover: #d4d4d4
            border_color_focus: #0284c7
            border_color_down: #0284c7
            border_color_disabled: #f5f5f5

            border_color_2: #e5e5e5
            border_color_2_hover: #d4d4d4
            border_color_2_focus: #0284c7
            border_color_2_down: #0284c7
            border_color_2_disabled: #f5f5f5

            arrow_color: #666666
            arrow_color_hover: #333333
            arrow_color_focus: #4A90D9
            arrow_color_down: #4A90D9
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Small size variant
    pub MpDropdownSmall = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 8, right: 20, top: 4, bottom: 4 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 12.0
            }
        }

        draw_bg: {
            border_size: 1.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #FFFFFF
            color_hover: #FAFAFA
            color_focus: #FFFFFF
            color_down: #F5F5F5
            color_disabled: #F5F5F5

            border_color: #e5e5e5
            border_color_hover: #d4d4d4
            border_color_focus: #0284c7
            border_color_down: #0284c7
            border_color_disabled: #f5f5f5

            border_color_2: #e5e5e5
            border_color_2_hover: #d4d4d4
            border_color_2_focus: #0284c7
            border_color_2_down: #0284c7
            border_color_2_disabled: #f5f5f5

            arrow_color: #666666
            arrow_color_hover: #333333
            arrow_color_focus: #4A90D9
            arrow_color_down: #4A90D9
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Large size variant
    pub MpDropdownLarge = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 16, right: 28, top: 12, bottom: 12 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 16.0
            }
        }

        draw_bg: {
            border_size: 1.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #FFFFFF
            color_hover: #FAFAFA
            color_focus: #FFFFFF
            color_down: #F5F5F5
            color_disabled: #F5F5F5

            border_color: #e5e5e5
            border_color_hover: #d4d4d4
            border_color_focus: #0284c7
            border_color_down: #0284c7
            border_color_disabled: #f5f5f5

            border_color_2: #e5e5e5
            border_color_2_hover: #d4d4d4
            border_color_2_focus: #0284c7
            border_color_2_down: #0284c7
            border_color_2_disabled: #f5f5f5

            arrow_color: #666666
            arrow_color_hover: #333333
            arrow_color_focus: #4A90D9
            arrow_color_down: #4A90D9
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 16.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Ghost variant - transparent background, no border
    pub MpDropdownGhost = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 0.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #00000000
            color_hover: #0000000D
            color_focus: #00000000
            color_down: #0000001A
            color_disabled: #00000000

            border_color: #00000000
            border_color_hover: #00000000
            border_color_focus: #00000000
            border_color_down: #00000000
            border_color_disabled: #00000000

            border_color_2: #00000000
            border_color_2_hover: #00000000
            border_color_2_focus: #00000000
            border_color_2_down: #00000000
            border_color_2_disabled: #00000000

            arrow_color: #666666
            arrow_color_hover: #333333
            arrow_color_focus: #0284c7
            arrow_color_down: #0284c7
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Outline variant - transparent background with border
    pub MpDropdownOutline = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 1.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #00000000
            color_hover: #0000000D
            color_focus: #00000000
            color_down: #0000001A
            color_disabled: #00000000

            border_color: #d4d4d4
            border_color_hover: #a3a3a3
            border_color_focus: #0284c7
            border_color_down: #0284c7
            border_color_disabled: #e5e5e5

            border_color_2: #d4d4d4
            border_color_2_hover: #a3a3a3
            border_color_2_focus: #0284c7
            border_color_2_down: #0284c7
            border_color_2_disabled: #e5e5e5

            arrow_color: #666666
            arrow_color_hover: #333333
            arrow_color_focus: #0284c7
            arrow_color_down: #0284c7
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Filled variant - light gray background
    pub MpDropdownFilled = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 0.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #f5f5f5
            color_hover: #e5e5e5
            color_focus: #f5f5f5
            color_down: #d4d4d4
            color_disabled: #fafafa

            border_color: #00000000
            border_color_hover: #00000000
            border_color_focus: #0284c7
            border_color_down: #0284c7
            border_color_disabled: #00000000

            border_color_2: #00000000
            border_color_2_hover: #00000000
            border_color_2_focus: #0284c7
            border_color_2_down: #0284c7
            border_color_2_disabled: #00000000

            arrow_color: #525252
            arrow_color_hover: #333333
            arrow_color_focus: #0284c7
            arrow_color_down: #0284c7
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Accent/Primary variant - blue accent color
    pub MpDropdownAccent = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #FFFFFF
            color_hover: #FFFFFF
            color_focus: #FFFFFF
            color_down: #FFFFFF
            color_disabled: #94a3b8

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 0.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #0284c7
            color_hover: #0369a1
            color_focus: #0284c7
            color_down: #075985
            color_disabled: #bae6fd

            border_color: #00000000
            border_color_hover: #00000000
            border_color_focus: #00000000
            border_color_down: #00000000
            border_color_disabled: #00000000

            border_color_2: #00000000
            border_color_2_hover: #00000000
            border_color_2_focus: #00000000
            border_color_2_down: #00000000
            border_color_2_disabled: #00000000

            arrow_color: #FFFFFF
            arrow_color_hover: #FFFFFF
            arrow_color_focus: #FFFFFF
            arrow_color_down: #FFFFFF
            arrow_color_disabled: #94a3b8
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    // Danger variant - red color for destructive actions
    pub MpDropdownDanger = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #FFFFFF
            color_hover: #FFFFFF
            color_focus: #FFFFFF
            color_down: #FFFFFF
            color_disabled: #fca5a5

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 0.0
            border_radius: (RADIUS_SMALL)
            color_dither: 0.0

            color: #dc2626
            color_hover: #b91c1c
            color_focus: #dc2626
            color_down: #991b1b
            color_disabled: #fecaca

            border_color: #00000000
            border_color_hover: #00000000
            border_color_focus: #00000000
            border_color_down: #00000000
            border_color_disabled: #00000000

            border_color_2: #00000000
            border_color_2_hover: #00000000
            border_color_2_focus: #00000000
            border_color_2_down: #00000000
            border_color_2_disabled: #00000000

            arrow_color: #FFFFFF
            arrow_color_hover: #FFFFFF
            arrow_color_focus: #FFFFFF
            arrow_color_down: #FFFFFF
            arrow_color_disabled: #fca5a5
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: (RADIUS_MEDIUM)
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }
}
